// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! `usbnexus` command line tool.

#[cfg(unix)]
mod admin;
mod ui;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use usbnexus_core::client::{self, AttachEvent, ClientConfig, ClientError, Target};
use usbnexus_core::identity::{short_fingerprint, Identity};
use usbnexus_core::trust::TrustStore;
use usbnexus_core::{discovery, DEFAULT_PORT};
use usbnexus_i18n::t;

#[derive(Parser)]
#[command(name = "usbnexus", version)]
struct Cli {
    #[arg(long, global = true, value_name = "CODE")]
    lang: Option<String>,
    #[arg(long, global = true, value_name = "DIR")]
    state_dir: Option<PathBuf>,
    #[arg(long, global = true)]
    name: Option<String>,
    #[arg(short, long, global = true)]
    verbose: bool,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Serve {
        #[arg(long, value_name = "BUSID")]
        export: Vec<String>,
        #[arg(long, value_name = "ADDR", default_value_t = format!("0.0.0.0:{DEFAULT_PORT}"))]
        listen: String,
        #[arg(long)]
        pair: bool,
        #[arg(long)]
        no_mdns: bool,
    },
    Pin {
        #[arg(long, default_value_t = 300)]
        seconds: u64,
    },
    Local,
    Discover {
        #[arg(long, default_value_t = 3)]
        timeout: u64,
    },
    Pair {
        server: String,
        #[arg(long)]
        pin: Option<String>,
    },
    List {
        server: String,
    },
    Attach {
        server: String,
        busid: String,
    },
    Peers,
    Forget {
        peer: String,
    },
}

/// Paths and identity shared by all commands.
struct Ctx {
    dir: PathBuf,
    name: String,
}

impl Ctx {
    fn identity(&self) -> Result<Identity> {
        Identity::load_or_create(&self.dir, &self.name)
    }

    /// Servers this computer has paired with.
    fn servers(&self) -> Result<TrustStore> {
        TrustStore::load(&self.dir.join("trusted-servers.json"))
    }

    /// Clients allowed to use this computer's devices.
    fn clients(&self) -> Result<TrustStore> {
        TrustStore::load(&self.dir.join("trusted-clients.json"))
    }

    fn client_config(&self) -> Result<ClientConfig> {
        Ok(ClientConfig { name: self.name.clone(), identity: self.identity()?, trust: self.servers()? })
    }

    #[cfg(unix)]
    fn admin_socket(&self) -> PathBuf {
        self.dir.join("admin.sock")
    }
}

fn is_root() -> bool {
    #[cfg(unix)]
    {
        // SAFETY: geteuid has no preconditions and cannot fail.
        unsafe { libc::geteuid() == 0 }
    }
    #[cfg(not(unix))]
    {
        false
    }
}

fn default_state_dir() -> PathBuf {
    if cfg!(target_os = "linux") && is_root() {
        return PathBuf::from("/var/lib/usbnexus");
    }
    directories::ProjectDirs::from("org", "usbnexus", "usbnexus")
        .map(|d| d.data_dir().to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".usbnexus"))
}

fn default_name() -> String {
    let from_file = |p: &str| std::fs::read_to_string(p).ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
    from_file("/proc/sys/kernel/hostname")
        .or_else(|| from_file("/etc/hostname"))
        .or_else(|| std::env::var("COMPUTERNAME").ok())
        .or_else(|| std::env::var("HOSTNAME").ok())
        .unwrap_or_else(|| "usbnexus".into())
}

/// Finds `--lang` before clap runs, so help output is already translated.
fn lang_from_args() -> Option<String> {
    let args: Vec<String> = std::env::args().collect();
    args.iter().enumerate().find_map(|(i, a)| {
        a.strip_prefix("--lang=")
            .map(str::to_string)
            .or_else(|| (a == "--lang").then(|| args.get(i + 1).cloned()).flatten())
    })
}

fn main() -> ExitCode {
    usbnexus_i18n::init(usbnexus_i18n::detect(lang_from_args().as_deref()));
    let matches = ui::localize(Cli::command(), true).get_matches();
    let cli = match Cli::from_arg_matches(&matches) {
        Ok(c) => c,
        Err(e) => e.exit(),
    };

    let filter = if cli.verbose { "usbnexus_core=debug,usbnexus=debug" } else { "error" };
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into()))
        .with_writer(std::io::stderr)
        .init();

    let ctx = Ctx {
        dir: cli.state_dir.clone().unwrap_or_else(default_state_dir),
        name: cli.name.clone().unwrap_or_else(default_name),
    };
    let target = match &cli.command {
        Cmd::Pair { server, .. } | Cmd::List { server } | Cmd::Attach { server, .. } => Some(server.clone()),
        _ => None,
    };

    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    match rt.block_on(run(&ctx, cli.command)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", ui::describe(&e, target.as_deref()));
            ExitCode::FAILURE
        }
    }
}

async fn run(ctx: &Ctx, cmd: Cmd) -> Result<()> {
    match cmd {
        Cmd::Serve { export, listen, pair, no_mdns } => serve(ctx, export, &listen, pair, no_mdns).await,
        Cmd::Pin { seconds } => pin(ctx, seconds).await,
        Cmd::Local => local(),
        Cmd::Discover { timeout } => discover(ctx, timeout).await,
        Cmd::Pair { server, pin } => pair(ctx, &server, pin).await,
        Cmd::List { server } => list(ctx, &server).await,
        Cmd::Attach { server, busid } => attach(ctx, &server, &busid).await,
        Cmd::Peers => peers(ctx),
        Cmd::Forget { peer } => forget(ctx, &peer),
    }
}

/// Resolves on Ctrl+C, or on SIGTERM (e.g. from systemd) on Unix.
async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        if let Ok(mut term) = signal(SignalKind::terminate()) {
            tokio::select! {
                _ = tokio::signal::ctrl_c() => {}
                _ = term.recv() => {}
            }
            return;
        }
    }
    let _ = tokio::signal::ctrl_c().await;
}

fn require_root() -> Result<()> {
    if cfg!(target_os = "linux") && !is_root() {
        bail!(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "root required"));
    }
    Ok(())
}

fn hex4(v: u16) -> String {
    format!("{v:04x}")
}

#[cfg(target_os = "linux")]
async fn serve(ctx: &Ctx, export: Vec<String>, listen: &str, pair: bool, no_mdns: bool) -> Result<()> {
    use usbnexus_core::linux::{read_device, LinuxExport};
    use usbnexus_core::server::{Server, ServerConfig, ServerEvent};

    require_root()?;
    let sys = Path::new("/sys");
    let identity = ctx.identity()?;
    let backend = Arc::new(LinuxExport::new(sys, export.clone()));
    let events = Arc::new(|ev: ServerEvent| match ev {
        ServerEvent::Paired { name, .. } => println!("{}", t!("serve-paired", name = name)),
        ServerEvent::PairingFailed { addr } => println!("{}", t!("serve-pairing-failed", addr = addr.to_string())),
        ServerEvent::Exported { busid, client } => println!("{}", t!("serve-exported", busid = busid, client = client)),
        ServerEvent::Released { busid, client } => println!("{}", t!("serve-released", busid = busid, client = client)),
    });
    let server = Server::with_events(
        ServerConfig {
            name: ctx.name.clone(),
            identity: identity.clone(),
            trust: ctx.clients()?,
            backend: backend.clone(),
        },
        events,
    )?;

    let listener = tokio::net::TcpListener::bind(listen).await.with_context(|| format!("listening on {listen}"))?;
    let port = listener.local_addr()?.port();
    println!("{}", t!("serve-started", name = ctx.name.as_str(), addr = listen));
    println!("{}", t!("serve-fingerprint", fp = short_fingerprint(server.fingerprint())));

    if export.is_empty() {
        println!("{}", t!("serve-no-exports"));
    } else {
        println!("{}", t!("serve-exporting"));
        let rows: Vec<Vec<String>> = export
            .iter()
            .map(|b| match read_device(sys, b) {
                Ok(d) => vec![
                    b.clone(),
                    format!("{}:{}", hex4(d.info.id_vendor), hex4(d.info.id_product)),
                    d.product.unwrap_or_default(),
                ],
                Err(e) => vec![b.clone(), "-".into(), format!("{e:#}")],
            })
            .collect();
        ui::table(&[t!("col-busid"), t!("col-id"), t!("col-product")], &rows);
    }

    let _mdns = if no_mdns {
        None
    } else {
        match discovery::advertise(&ctx.name, port, server.fingerprint()) {
            Ok(a) => Some(a),
            Err(e) => {
                println!("{}", t!("serve-mdns-failed", detail = format!("{e:#}")));
                None
            }
        }
    };

    admin::listen(&ctx.admin_socket(), server.clone())?;
    if pair {
        let seconds = 300;
        let pin = server.open_pairing(Duration::from_secs(seconds));
        println!("{}", t!("serve-pairing-pin", pin = pin, seconds = seconds));
    }

    let result = tokio::select! {
        r = server.serve(listener) => r,
        _ = shutdown_signal() => Ok(()),
    };
    println!("{}", t!("serve-stopping"));
    for b in &export {
        if let Err(e) = backend.release(b) {
            eprintln!("{}", t!("serve-bind-failed", busid = b.as_str(), detail = format!("{e:#}")));
        }
    }
    let _ = std::fs::remove_file(ctx.admin_socket());
    result
}

#[cfg(not(target_os = "linux"))]
async fn serve(_: &Ctx, _: Vec<String>, _: &str, _: bool, _: bool) -> Result<()> {
    bail!(t!("unsupported-os"))
}

#[cfg(unix)]
async fn pin(ctx: &Ctx, seconds: u64) -> Result<()> {
    use admin::{Reply, Request};
    let reply = match admin::request(&ctx.admin_socket(), &Request::OpenPairing { seconds }).await {
        Ok(r) => r,
        Err(e)
            if e.downcast_ref::<std::io::Error>()
                .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied) =>
        {
            return Err(e)
        }
        Err(_) => bail!(t!("pin-no-server")),
    };
    match reply {
        Reply::Pairing { pin, seconds, name } => {
            println!("{}", t!("pin-show", pin = pin));
            println!("{}", t!("pin-hint", name = name, seconds = seconds));
            Ok(())
        }
        Reply::Error { message } => bail!(message),
    }
}

#[cfg(not(unix))]
async fn pin(_: &Ctx, _: u64) -> Result<()> {
    bail!(t!("unsupported-os"))
}

#[cfg(target_os = "linux")]
fn local() -> Result<()> {
    let devices = usbnexus_core::linux::list_local(Path::new("/sys"))?;
    if devices.is_empty() {
        println!("{}", t!("local-empty"));
        return Ok(());
    }
    println!("{}", t!("local-header"));
    let rows: Vec<Vec<String>> = devices
        .into_iter()
        .map(|d| {
            vec![
                d.info.busid.clone(),
                format!("{}:{}", hex4(d.info.id_vendor), hex4(d.info.id_product)),
                d.info.speed.label().to_string(),
                d.driver.unwrap_or_else(|| "-".into()),
                [d.manufacturer, d.product].into_iter().flatten().collect::<Vec<_>>().join(" "),
            ]
        })
        .collect();
    ui::table(&[t!("col-busid"), t!("col-id"), t!("col-speed"), t!("col-driver"), t!("col-product")], &rows);
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn local() -> Result<()> {
    bail!(t!("unsupported-os"))
}

async fn discover(ctx: &Ctx, timeout: u64) -> Result<()> {
    println!("{}", t!("discover-searching"));
    let found = discovery::browse(Duration::from_secs(timeout)).await?;
    if found.is_empty() {
        println!("{}", t!("discover-none"));
        return Ok(());
    }
    let trust = ctx.servers()?;
    let rows: Vec<Vec<String>> = found
        .into_iter()
        .map(|d| {
            vec![
                d.name,
                short_fingerprint(&d.fingerprint),
                d.addrs.first().map(|a| a.to_string()).unwrap_or_default(),
                if trust.is_trusted(&d.fingerprint) { t!("yes") } else { t!("no") },
            ]
        })
        .collect();
    ui::table(&[t!("col-name"), t!("col-fingerprint"), t!("col-address"), t!("discover-paired")], &rows);
    Ok(())
}

/// Resolves a server for pairing: a paired peer, a server announced on the
/// LAN under that name, or a plain address.
async fn resolve_new(server: &str, trust: &TrustStore) -> Result<String> {
    if let Target::Peer { .. } = Target::parse(server, trust) {
        return Target::parse(server, trust).resolve(trust).await;
    }
    if let Ok(found) = discovery::browse(Duration::from_secs(2)).await {
        if let Some(d) = found.into_iter().find(|d| d.name.eq_ignore_ascii_case(server)) {
            if let Some(a) = d.addrs.first() {
                return Ok(a.to_string());
            }
        }
    }
    Ok(server.to_string())
}

fn prompt_pin() -> Result<String> {
    print!("{} ", t!("pair-enter-pin"));
    std::io::stdout().flush()?;
    let mut line = String::new();
    std::io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

async fn pair(ctx: &Ctx, server: &str, pin: Option<String>) -> Result<()> {
    let cfg = ctx.client_config()?;
    let addr = resolve_new(server, &cfg.trust).await?;
    match client::connect(&cfg, &addr, None).await {
        Ok(s) => {
            println!("{}", t!("pair-already", name = s.server_name));
            return Ok(());
        }
        Err(e) if matches!(e.downcast_ref::<ClientError>(), Some(ClientError::PairingRequired { .. })) => {}
        Err(e) => return Err(e),
    }
    let pin = match pin {
        Some(p) => p,
        None => tokio::task::spawn_blocking(prompt_pin).await??,
    };
    let s = client::connect(&cfg, &addr, Some(&pin)).await?;
    println!("{}", t!("pair-ok", name = s.server_name, fp = short_fingerprint(&s.server_fingerprint)));
    Ok(())
}

async fn list(ctx: &Ctx, server: &str) -> Result<()> {
    let cfg = ctx.client_config()?;
    let addr = Target::parse(server, &cfg.trust).resolve(&cfg.trust).await?;
    let mut s = client::connect(&cfg, &addr, None).await?;
    let devices = s.list().await?;
    if devices.is_empty() {
        println!("{}", t!("list-empty"));
        return Ok(());
    }
    println!("{}", t!("list-header", name = s.server_name.as_str()));
    let rows: Vec<Vec<String>> = devices
        .into_iter()
        .map(|d| {
            vec![
                d.info.busid.clone(),
                format!("{}:{}", hex4(d.info.id_vendor), hex4(d.info.id_product)),
                d.info.speed.label().to_string(),
                [d.manufacturer, d.product].into_iter().flatten().collect::<Vec<_>>().join(" "),
                if d.in_use { t!("list-in-use") } else { String::new() },
            ]
        })
        .collect();
    ui::table(&[t!("col-busid"), t!("col-id"), t!("col-speed"), t!("col-product"), t!("col-state")], &rows);
    Ok(())
}

#[cfg(target_os = "linux")]
async fn attach(ctx: &Ctx, server: &str, busid: &str) -> Result<()> {
    require_root()?;
    let cfg = ctx.client_config()?;
    let target = Target::parse(server, &cfg.trust);
    let backend = Arc::new(usbnexus_core::linux::LinuxImport::new("/sys"));
    let events = |ev: AttachEvent| match ev {
        AttachEvent::Connecting { addr } => println!("{}", t!("attach-connecting", addr = addr)),
        AttachEvent::Attached { port, .. } => {
            println!("{}", t!("attach-attached", busid = busid, port = port));
            println!("{}", t!("attach-stop-hint"));
        }
        AttachEvent::Disconnected { reason } => println!("{}", t!("attach-disconnected", reason = reason)),
        AttachEvent::Retrying { delay } => println!("{}", t!("attach-retrying", seconds = delay.as_secs_f64().ceil())),
        AttachEvent::Detached => println!("{}", t!("attach-detached")),
    };
    tokio::select! {
        r = client::attach_forever(&cfg, &target, busid, backend, events) => r,
        _ = shutdown_signal() => {
            // Dropping the relay closes the socket; the kernel detaches the device.
            println!("{}", t!("attach-detached"));
            Ok(())
        }
    }
}

#[cfg(not(target_os = "linux"))]
async fn attach(_: &Ctx, _: &str, _: &str) -> Result<()> {
    bail!(t!("unsupported-os"))
}

fn peer_rows(store: &TrustStore) -> Vec<Vec<String>> {
    store
        .peers()
        .into_iter()
        .map(|p| vec![p.name, short_fingerprint(&p.fingerprint), p.last_addr.unwrap_or_default()])
        .collect()
}

fn peers(ctx: &Ctx) -> Result<()> {
    let (servers, clients) = (peer_rows(&ctx.servers()?), peer_rows(&ctx.clients()?));
    if servers.is_empty() && clients.is_empty() {
        println!("{}", t!("peers-empty"));
        return Ok(());
    }
    let headers = [t!("col-name"), t!("col-fingerprint"), t!("col-address")];
    for (title, rows) in [("peers-servers", servers), ("peers-clients", clients)] {
        if !rows.is_empty() {
            println!("{}", t!(title));
            ui::table(&headers, &rows);
        }
    }
    Ok(())
}

fn forget(ctx: &Ctx, peer: &str) -> Result<()> {
    let mut removed = None;
    for store in [ctx.servers()?, ctx.clients()?] {
        if let Some(p) = store.find(peer) {
            store.remove(&p.fingerprint)?;
            removed = Some(p.name);
        }
    }
    match removed {
        Some(name) => println!("{}", t!("forget-ok", name = name)),
        None => bail!(t!("forget-unknown", peer = peer)),
    }
    Ok(())
}
