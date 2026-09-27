// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! `usbnexus` command line tool.

// Device commands are Linux-only until the Windows/macOS backends land;
// their helpers are unused elsewhere.
#![cfg_attr(not(target_os = "linux"), allow(unused_imports, dead_code, unreachable_code))]

mod ui;
#[cfg(windows)]
mod winservice;

use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
use usbnexus_core::client::{self, AttachEvent, ClientConfig, ClientError, Target};
use usbnexus_core::device_id::DeviceId;
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
    #[arg(long, global = true, value_name = "PATH")]
    socket: Option<PathBuf>,
    #[arg(short, long, global = true)]
    verbose: bool,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(clap::Args)]
struct ServeOpts {
    #[arg(long, value_name = "BUSID")]
    export: Vec<String>,
    #[arg(long, value_name = "ADDR", default_value_t = format!("0.0.0.0:{DEFAULT_PORT}"))]
    listen: String,
    #[arg(long)]
    pair: bool,
    #[arg(long)]
    no_mdns: bool,
    #[arg(long)]
    allow_all_users: bool,
    /// Simulated devices, for trying the interface without hardware.
    #[arg(long, hide = true)]
    demo: bool,
}

impl ServeOpts {
    /// Settings used when started by the Windows service manager.
    #[cfg(windows)]
    fn service_defaults() -> Self {
        ServeOpts {
            export: vec![],
            listen: format!("0.0.0.0:{DEFAULT_PORT}"),
            pair: false,
            no_mdns: false,
            allow_all_users: false,
            demo: false,
        }
    }
}

#[cfg(windows)]
#[derive(Subcommand)]
enum ServiceCmd {
    Install(InstallOpts),
    Uninstall,
    /// Entry point for the Windows service manager.
    #[command(hide = true)]
    Run,
}

/// Settings the installer passes to `service install`; omitted web options
/// keep their current values.
#[cfg(windows)]
#[derive(clap::Args, Default)]
pub struct InstallOpts {
    /// Do not set up sharing this computer's devices.
    #[arg(long)]
    no_server: bool,
    /// Do not set up using other computers' devices.
    #[arg(long)]
    no_client: bool,
    #[arg(long, value_enum)]
    web: Option<WebAccessArg>,
    #[arg(long)]
    web_port: Option<u16>,
    /// File holding the new web password (read, then left to the caller to delete).
    #[arg(long, value_name = "FILE")]
    web_password_file: Option<PathBuf>,
}

#[cfg(windows)]
#[derive(Clone, Copy, clap::ValueEnum)]
enum WebAccessArg {
    Off,
    /// Only from this computer.
    Local,
    /// From the whole network.
    Network,
}

#[derive(Clone, Copy, clap::ValueEnum)]
enum PolicyArg {
    Open,
    Restricted,
}

#[derive(Subcommand)]
enum WebCmd {
    Enable {
        #[arg(long)]
        lan: bool,
        #[arg(long)]
        port: Option<u16>,
    },
    Disable,
    Password,
    Status,
    /// Exit status 0 if the web interface could use this port, 1 if not.
    #[command(hide = true)]
    CheckPort {
        port: u16,
    },
    /// Exit status 0 if a web password is set (for the installer).
    #[command(hide = true)]
    HasPassword,
}

#[derive(Subcommand)]
enum Cmd {
    Daemon(ServeOpts),
    Web {
        #[command(subcommand)]
        action: WebCmd,
    },
    #[cfg(windows)]
    Service {
        #[command(subcommand)]
        action: ServiceCmd,
    },
    Serve(ServeOpts),
    Pin {
        #[arg(long, default_value_t = 300)]
        seconds: u64,
    },
    Local,
    /// Lets a user control the service from the desktop app (Linux).
    #[cfg(target_os = "linux")]
    AllowUser {
        user: String,
    },
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
        device: String,
    },
    Peers,
    Policy {
        #[arg(value_enum)]
        policy: Option<PolicyArg>,
    },
    History {
        #[arg(long)]
        csv: bool,
        #[arg(long, default_value_t = 50)]
        limit: usize,
    },
    Forget {
        peer: String,
    },
    /// Sends a raw JSON request to the running service (for scripts).
    #[command(hide = true)]
    Api {
        request: String,
    },
}

/// Paths and identity shared by all commands.
struct Ctx {
    dir: PathBuf,
    name: String,
    /// Local API socket of the service.
    socket: PathBuf,
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
    #[cfg(windows)]
    {
        // Shared by the service (LocalSystem) and administrators.
        let base =
            std::env::var_os("ProgramData").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"));
        return base.join("USB Nexus");
    }
    #[allow(unreachable_code)]
    if cfg!(target_os = "linux") && is_root() {
        return PathBuf::from("/var/lib/usbnexus");
    }
    if cfg!(target_os = "macos") && is_root() {
        return PathBuf::from("/Library/Application Support/USB Nexus");
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

    let ctx = Ctx {
        dir: cli.state_dir.clone().unwrap_or_else(default_state_dir),
        name: cli.name.clone().unwrap_or_else(default_name),
        socket: cli.socket.clone().unwrap_or_else(usbnexus_core::api::default_socket),
    };

    let filter = if cli.verbose { "usbnexus_core=debug,usbnexus=debug" } else { "error" };
    let env_filter = || tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| filter.into());
    #[cfg(windows)]
    if matches!(cli.command, Cmd::Service { action: ServiceCmd::Run }) {
        // A service has no console: log to a file next to its state.
        let _ = std::fs::create_dir_all(&ctx.dir);
        let file = std::fs::OpenOptions::new().create(true).append(true).open(ctx.dir.join("service.log"));
        if let Ok(file) = file {
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::new("usbnexus_core=info,usbnexus=info"))
                .with_ansi(false)
                .with_writer(std::sync::Mutex::new(file))
                .init();
        }
        // A panic would otherwise go to a console nobody sees and could leave
        // a half-working service behind (e.g. a dead local API task). Log it
        // and exit so the service manager restarts the service.
        std::panic::set_hook(Box::new(|info| {
            let backtrace = std::backtrace::Backtrace::force_capture();
            tracing::error!("panic: {info}\n{backtrace}");
            std::process::exit(1);
        }));
        return match winservice::run(ctx) {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                tracing::error!("{e:#}");
                ExitCode::FAILURE
            }
        };
    }
    tracing_subscriber::fmt().with_env_filter(env_filter()).with_writer(std::io::stderr).init();
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
        Cmd::Daemon(opts) | Cmd::Serve(opts) => run_daemon(ctx, opts, shutdown_signal()).await,
        Cmd::Web { action } => web(ctx, action).await,
        #[cfg(windows)]
        Cmd::Service { action: ServiceCmd::Install(opts) } => winservice::install(ctx, opts).await,
        #[cfg(windows)]
        Cmd::Service { action: ServiceCmd::Uninstall } => winservice::uninstall(),
        #[cfg(windows)]
        Cmd::Service { action: ServiceCmd::Run } => unreachable!("handled in main"),
        Cmd::Pin { seconds } => pin(ctx, seconds).await,
        Cmd::Local => local(),
        #[cfg(target_os = "linux")]
        Cmd::AllowUser { user } => allow_user(ctx, &user),
        Cmd::Discover { timeout } => discover(ctx, timeout).await,
        Cmd::Pair { server, pin } => pair(ctx, &server, pin).await,
        Cmd::List { server } => list(ctx, &server).await,
        Cmd::Attach { server, device } => attach(ctx, &server, &device).await,
        Cmd::Peers => peers(ctx),
        Cmd::Policy { policy } => policy_cmd(ctx, policy).await,
        Cmd::History { csv, limit } => history(ctx, csv, limit).await,
        Cmd::Forget { peer } => forget(ctx, &peer),
        Cmd::Api { request } => {
            let req: usbnexus_core::api::Request = serde_json::from_str(&request).context("parsing request")?;
            let v: serde_json::Value = usbnexus_core::api::call(&ctx.socket, &req).await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            Ok(())
        }
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

/// Linux: the usbip kernel modules each role needs, and how to get them.
#[cfg(target_os = "linux")]
mod linux_setup {
    use std::path::Path;
    use std::sync::Mutex;
    use std::time::{Duration, Instant};

    use usbnexus_core::api::{Roles, SetupIssue};

    /// Loading is retried this often while a module is missing.
    const MODPROBE_INTERVAL: Duration = Duration::from_secs(30);

    fn loaded(module: &str) -> bool {
        match module {
            "usbip-host" => Path::new("/sys/bus/usb/drivers/usbip-host").is_dir(),
            "vhci-hcd" => Path::new("/sys/devices/platform/vhci_hcd.0").is_dir(),
            _ => false,
        }
    }

    /// The command that installs the modules on this distribution, if known.
    fn install_command() -> Option<String> {
        let os = std::fs::read_to_string("/etc/os-release").ok()?;
        let field = |name: &str| {
            os.lines()
                .find_map(|l| l.strip_prefix(name)?.strip_prefix('='))
                .map(|v| v.trim_matches('"').to_lowercase())
                .unwrap_or_default()
        };
        let ids = format!("{} {}", field("ID"), field("ID_LIKE"));
        if ids.contains("ubuntu") {
            let kernel = std::fs::read_to_string("/proc/sys/kernel/osrelease").ok()?;
            return Some(format!("sudo apt install linux-modules-extra-{}", kernel.trim()));
        }
        if ["fedora", "rhel", "centos", "rocky", "almalinux"].iter().any(|d| ids.contains(d)) {
            return Some("sudo dnf install kernel-modules-extra".to_string());
        }
        None
    }

    pub fn issues(roles: Roles) -> Vec<SetupIssue> {
        static LAST_TRY: Mutex<Option<Instant>> = Mutex::new(None);
        let wanted: Vec<&str> = [(roles.server, "usbip-host"), (roles.client, "vhci-hcd")]
            .into_iter()
            .filter_map(|(on, m)| on.then_some(m))
            .collect();
        let mut missing: Vec<&str> = wanted.iter().copied().filter(|m| !loaded(m)).collect();
        if !missing.is_empty() {
            // Installed since the service started? Try loading again.
            let mut last = LAST_TRY.lock().unwrap();
            if last.map_or(true, |t| t.elapsed() >= MODPROBE_INTERVAL) {
                *last = Some(Instant::now());
                for m in &missing {
                    let _ = std::process::Command::new("modprobe").arg(m).output();
                }
                missing.retain(|m| !loaded(m));
            }
        }
        if missing.is_empty() {
            return vec![];
        }
        vec![SetupIssue {
            code: "kernel_modules_missing".into(),
            detail: missing.join(", "),
            command: install_command(),
        }]
    }
}

/// Adds `user` to the `usbnexus` group so the desktop app may control the
/// service, and grants access to the running service's socket right away
/// (group membership only applies to new sessions).
#[cfg(target_os = "linux")]
fn allow_user(ctx: &Ctx, user: &str) -> Result<()> {
    require_root()?;
    if user.is_empty() || !user.chars().all(|c| c.is_ascii_alphanumeric() || "._-".contains(c)) {
        bail!(t!("allow-user-invalid", user = user));
    }
    let status =
        std::process::Command::new("usermod").args(["-aG", "usbnexus", user]).status().context("running usermod")?;
    if !status.success() {
        bail!(t!("allow-user-failed", user = user));
    }
    let socket = &ctx.socket;
    let now = socket.exists()
        && std::process::Command::new("setfacl")
            .arg("-m")
            .arg(format!("u:{user}:rw"))
            .arg(socket)
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
    println!("{}", t!("allow-user-done", user = user));
    if !now {
        println!("{}", t!("allow-user-relogin"));
    }
    Ok(())
}

fn hex4(v: u16) -> String {
    format!("{v:04x}")
}

/// Runs the service in the foreground. `serve` is the same with devices to
/// share given on the command line (they are remembered).
/// Picks the platform backends for the service.
type Backends = (Arc<dyn usbnexus_core::backend::DeviceHost>, Arc<dyn usbnexus_core::backend::ImportBackend>);

/// Backends for `roles`: a computer set up for one role only does not touch
/// the other's drivers (e.g. no hub queries and no VBoxUSB on a client).
fn backends_for(demo: bool, roles: usbnexus_core::api::Roles) -> Result<Backends> {
    let (mut host, mut import) = backends(demo)?;
    if !roles.server {
        host = Arc::new(usbnexus_core::backend::NoHost);
    }
    if !roles.client {
        import = Arc::new(usbnexus_core::backend::UnsupportedImport);
    }
    Ok((host, import))
}

fn backends(demo: bool) -> Result<Backends> {
    use usbnexus_core::backend::demo::{DemoHost, DemoImport};
    if demo {
        return Ok((Arc::new(DemoHost), Arc::new(DemoImport::default())));
    }
    #[cfg(target_os = "linux")]
    {
        use usbnexus_core::linux::{LinuxHost, LinuxImport};
        require_root()?;
        Ok((Arc::new(LinuxHost::new("/sys")), Arc::new(LinuxImport::new("/sys"))))
    }
    #[cfg(windows)]
    {
        use usbnexus_core::windows::WindowsImport;
        use usbnexus_core::windows_host::WindowsHost;
        Ok((Arc::new(WindowsHost), Arc::new(WindowsImport::default())))
    }
    #[cfg(target_os = "macos")]
    {
        // Sharing through libusb; macOS cannot use remote devices.
        use usbnexus_core::backend::UnsupportedImport;
        use usbnexus_core::libusb_host::LibusbHost;
        Ok((Arc::new(LibusbHost), Arc::new(UnsupportedImport)))
    }
    #[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
    {
        bail!(t!("unsupported-os"))
    }
}

/// Opens the local API endpoint; the returned guard removes it on drop.
#[cfg(unix)]
fn serve_api(ctx: &Ctx, opts: &ServeOpts, d: usbnexus_core::daemon::Daemon) -> Result<impl Drop> {
    struct Cleanup(PathBuf);
    impl Drop for Cleanup {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }
    let socket = ctx.socket.clone();
    if let Some(dir) = socket.parent() {
        std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
    }
    let _ = std::fs::remove_file(&socket);
    let listener = tokio::net::UnixListener::bind(&socket).with_context(|| format!("binding {}", socket.display()))?;
    secure_socket(&socket, opts.allow_all_users)?;
    tokio::spawn(usbnexus_core::api::serve(listener, d));
    Ok(Cleanup(socket))
}

#[cfg(windows)]
fn serve_api(ctx: &Ctx, opts: &ServeOpts, d: usbnexus_core::daemon::Daemon) -> Result<impl Drop> {
    struct Nothing;
    impl Drop for Nothing {
        fn drop(&mut self) {}
    }
    tokio::spawn(usbnexus_core::api::serve_pipe(&ctx.socket, d, opts.allow_all_users)?);
    Ok(Nothing)
}

/// Runs the service until `stop` resolves. `serve` is the same with devices
/// to share given on the command line (they are remembered).
async fn run_daemon(ctx: &Ctx, opts: ServeOpts, stop: impl std::future::Future<Output = ()>) -> Result<()> {
    use usbnexus_core::api::{Request, Response};
    use usbnexus_core::daemon::{Daemon, DaemonOptions};
    use usbnexus_core::server::ServerEvent;

    let (host, import) = backends_for(opts.demo, usbnexus_core::daemon::saved_roles(&ctx.dir))?;
    let label = |u: &usbnexus_core::server::DeviceUse| u.device_name.clone().unwrap_or_else(|| u.device.clone());
    let events = Arc::new(move |ev: ServerEvent| match ev {
        ServerEvent::Paired { name, .. } => println!("{}", t!("serve-paired", name = name)),
        ServerEvent::PairingFailed { addr, .. } => {
            println!("{}", t!("serve-pairing-failed", addr = addr.to_string()))
        }
        ServerEvent::Exported(u) => println!("{}", t!("serve-exported", busid = label(&u), client = u.client)),
        ServerEvent::Released { usage: u, .. } => {
            println!("{}", t!("serve-released", busid = label(&u), client = u.client))
        }
        ServerEvent::Denied(u) => println!("{}", t!("serve-denied", busid = label(&u), client = u.client)),
    });
    let d = Daemon::start(DaemonOptions {
        state_dir: ctx.dir.clone(),
        name: ctx.name.clone(),
        listen: opts.listen.clone(),
        mdns: !opts.no_mdns,
        host,
        import,
        events: Some(events),
    })
    .await?;
    let demo = opts.demo;
    #[cfg(target_os = "linux")]
    if !demo {
        d.set_setup_check(Arc::new(linux_setup::issues));
    }
    d.set_backend_factory(Arc::new(move |roles| {
        // Install what the new roles need (drivers) before using them.
        #[cfg(windows)]
        let reboot = if demo { false } else { winservice::prepare_roles(roles)? };
        #[cfg(not(windows))]
        let reboot = false;
        let (host, import) = backends_for(demo, roles)?;
        Ok((host, import, reboot))
    }));

    for busid in &opts.export {
        if let Response::Error { error } = d.handle(Request::SetShared { device: busid.clone(), shared: true }).await {
            eprintln!("{}", t!("serve-bind-failed", busid = busid.as_str(), detail = error.message));
        }
    }
    let _api = serve_api(ctx, &opts, d.clone())?;

    println!("{}", t!("serve-started", name = ctx.name.as_str(), addr = opts.listen.as_str()));
    println!("{}", t!("serve-fingerprint", fp = short_fingerprint(d.server().fingerprint())));
    let shared: Vec<Vec<String>> = match d.handle(Request::LocalDevices).await {
        Response::Ok { data } => serde_json::from_value::<Vec<usbnexus_core::api::LocalDeviceView>>(data)
            .unwrap_or_default()
            .into_iter()
            .filter(|v| v.shared)
            .map(|v| {
                vec![
                    v.busid.unwrap_or_else(|| t!("state-unplugged")),
                    format!("{}:{}", hex4(v.vendor_id), hex4(v.product_id)),
                    [v.manufacturer, v.product].into_iter().flatten().collect::<Vec<_>>().join(" "),
                ]
            })
            .collect(),
        Response::Error { .. } => vec![],
    };
    if shared.is_empty() {
        println!("{}", t!("serve-no-exports"));
    } else {
        println!("{}", t!("serve-exporting"));
        ui::table(&[t!("col-busid"), t!("col-id"), t!("col-product")], &shared);
    }
    if opts.pair {
        let seconds = 300;
        let pin = d.server().open_pairing(Duration::from_secs(seconds));
        println!("{}", t!("serve-pairing-pin", pin = pin, seconds = seconds));
    }

    stop.await;
    println!("{}", t!("serve-stopping"));
    d.shutdown();
    Ok(())
}

/// Restricts the API socket: members of the `usbnexus` group (if it exists)
/// may use it, or everyone with `--allow-all-users`; otherwise only root.
#[cfg(unix)]
fn secure_socket(path: &Path, allow_all: bool) -> Result<()> {
    use std::os::unix::ffi::OsStrExt;
    use std::os::unix::fs::PermissionsExt;
    let mode = if allow_all {
        0o666
    } else if let Some(gid) =
        group_id("usbnexus").or_else(|| cfg!(target_os = "macos").then(|| group_id("admin")).flatten())
    {
        let c = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        // SAFETY: `c` is a valid NUL-terminated path; -1 keeps the owner.
        if unsafe { libc::chown(c.as_ptr(), u32::MAX, gid) } != 0 {
            return Err(std::io::Error::last_os_error()).context("setting socket group");
        }
        0o660
    } else {
        0o600
    };
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(mode))?;
    Ok(())
}

#[cfg(unix)]
fn group_id(name: &str) -> Option<u32> {
    std::fs::read_to_string("/etc/group").ok()?.lines().find_map(|l| {
        let mut f = l.split(':');
        (f.next()? == name).then(|| f.nth(1)?.parse().ok()).flatten()
    })
}

async fn pin(ctx: &Ctx, seconds: u64) -> Result<()> {
    use usbnexus_core::api::{self, PairingView, Request, StatusView};
    let not_running = |e: anyhow::Error| -> anyhow::Error {
        let denied =
            e.downcast_ref::<std::io::Error>().is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied);
        if denied {
            e
        } else {
            anyhow::anyhow!(t!("pin-no-server"))
        }
    };
    let status: StatusView = api::call(&ctx.socket, &Request::Status).await.map_err(not_running)?;
    let p: PairingView = api::call(&ctx.socket, &Request::OpenPairing { seconds }).await?;
    println!("{}", t!("pin-show", pin = p.pin));
    println!("{}", t!("pin-hint", name = status.name, seconds = p.remaining_secs));
    Ok(())
}

#[cfg(any(target_os = "linux", windows, target_os = "macos"))]
fn local() -> Result<()> {
    #[cfg(target_os = "linux")]
    let devices = usbnexus_core::linux::list_local(Path::new("/sys"))?;
    #[cfg(target_os = "macos")]
    let devices = {
        use usbnexus_core::backend::DeviceHost;
        usbnexus_core::libusb_host::LibusbHost.list_all()?
    };
    #[cfg(windows)]
    let devices = {
        use usbnexus_core::backend::DeviceHost;
        usbnexus_core::windows_host::WindowsHost.list_all()?
    };
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
                d.driver.clone().unwrap_or_else(|| "-".into()),
                [d.manufacturer.clone(), d.product.clone()].into_iter().flatten().collect::<Vec<_>>().join(" "),
                DeviceId::of(&d).to_string(),
            ]
        })
        .collect();
    ui::table(
        &[t!("col-busid"), t!("col-id"), t!("col-speed"), t!("col-driver"), t!("col-product"), t!("col-device-id")],
        &rows,
    );
    Ok(())
}

#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
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
    if let Some(addr) = trust.find(server).and_then(|p| p.last_addr) {
        return Ok(addr);
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
    let mut s = Target::parse(server, &cfg.trust).connect(&cfg, &|_| {}).await?;
    let devices = s.list().await?;
    if devices.is_empty() {
        println!("{}", t!("list-empty"));
        return Ok(());
    }
    println!("{}", t!("list-header", name = s.server_name.as_str()));
    let rows: Vec<Vec<String>> = devices
        .into_iter()
        .map(|d| {
            let state = if !d.allowed {
                t!("state-no-permission")
            } else if !d.present {
                t!("state-unplugged")
            } else if d.in_use {
                t!("list-in-use")
            } else {
                String::new()
            };
            let id = if d.id.is_empty() { d.info.busid.clone() } else { d.id };
            vec![
                if d.present { d.info.busid } else { "-".into() },
                format!("{}:{}", hex4(d.info.id_vendor), hex4(d.info.id_product)),
                if d.present { d.info.speed.label().to_string() } else { String::new() },
                [d.manufacturer, d.product].into_iter().flatten().collect::<Vec<_>>().join(" "),
                state,
                id,
            ]
        })
        .collect();
    ui::table(
        &[t!("col-busid"), t!("col-id"), t!("col-speed"), t!("col-product"), t!("col-state"), t!("col-device-id")],
        &rows,
    );
    Ok(())
}

#[cfg(any(target_os = "linux", windows))]
async fn attach(ctx: &Ctx, server: &str, device: &str) -> Result<()> {
    let cfg = ctx.client_config()?;
    let target = Target::parse(server, &cfg.trust);
    #[cfg(target_os = "linux")]
    let backend = {
        require_root()?;
        Arc::new(usbnexus_core::linux::LinuxImport::new("/sys"))
    };
    #[cfg(windows)]
    let backend = Arc::new(usbnexus_core::windows::WindowsImport::default());
    // Waiting for a device is reported once, not on every attempt.
    let waiting = std::cell::Cell::new(false);
    let queue_position = std::cell::Cell::new(0);
    let events = |ev: AttachEvent| match ev {
        AttachEvent::Connecting { .. } if waiting.get() => {}
        AttachEvent::Connecting { addr } => println!("{}", t!("attach-connecting", addr = addr)),
        AttachEvent::Retrying { .. } if waiting.get() => {}
        AttachEvent::Attached { port, .. } => {
            waiting.set(false);
            queue_position.set(0);
            println!("{}", t!("attach-attached", busid = device, port = port));
            println!("{}", t!("attach-stop-hint"));
        }
        AttachEvent::Disconnected { error } if error.code == "no_such_device" => {
            if !waiting.replace(true) {
                println!("{}", t!("attach-waiting-device"));
            }
        }
        AttachEvent::Disconnected { error } => {
            waiting.set(false);
            let reason = ui::describe(&error.clone().into(), None);
            println!("{}", t!("attach-disconnected", reason = reason));
        }
        AttachEvent::Retrying { delay } => println!("{}", t!("attach-retrying", seconds = delay.as_secs_f64().ceil())),
        // Printed when the place in the queue changes.
        AttachEvent::Queued { position } => {
            waiting.set(true);
            if queue_position.replace(position) != position {
                println!("{}", t!("attach-queued", position = position));
            }
        }
        AttachEvent::Detached => println!("{}", t!("attach-detached")),
    };
    tokio::select! {
        r = client::attach_forever(&cfg, &target, device, backend, events) => r,
        _ = shutdown_signal() => {
            // Dropping the relay closes the socket; the kernel detaches the device.
            println!("{}", t!("attach-detached"));
            Ok(())
        }
    }
}

#[cfg(not(any(target_os = "linux", windows)))]
async fn attach(_: &Ctx, _: &str, _: &str) -> Result<()> {
    bail!(t!("unsupported-os"))
}

/// Asks for a new web password twice, without echo.
fn prompt_new_password() -> Result<String> {
    let first = rpassword::prompt_password(format!("{} ", t!("web-password-prompt")))?;
    let second = rpassword::prompt_password(format!("{} ", t!("web-password-repeat")))?;
    if first != second {
        bail!(t!("web-password-mismatch"));
    }
    Ok(first)
}

fn print_web_status(s: &usbnexus_core::api::WebStatusView) {
    if !s.enabled {
        println!("{}", t!("web-off"));
        return;
    }
    if let Some(e) = &s.error {
        println!("{}", t!("web-not-running", detail = e.as_str()));
        return;
    }
    println!("{}", t!("web-on"));
    for u in &s.urls {
        println!("  {u}");
    }
    if !s.lan {
        println!("{}", t!("web-local-only"));
    }
    if let Some(fp) = &s.fingerprint {
        println!("{}", t!("web-fingerprint", fp = short_fingerprint(fp)));
    }
}

/// Whether the web interface could listen on `port`: nobody listens on it,
/// or it is already ours.
fn check_port(ctx: &Ctx, port: u16) -> Result<()> {
    // The service itself listens on DEFAULT_PORT.
    if port == 0 || port == DEFAULT_PORT {
        bail!("port {port} is reserved");
    }
    // Windows: read the listener tables. Listening on all interfaces, even
    // for a moment, would make Windows Firewall ask about the installer's
    // copy of this program.
    #[cfg(windows)]
    let free = !usbnexus_core::windows::tcp_port_listening(port);
    // Elsewhere, try to listen (on loopback only).
    #[cfg(not(windows))]
    let free = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).is_ok();
    if free {
        return Ok(());
    }
    let web = usbnexus_core::daemon::saved_web(&ctx.dir);
    if web.enabled && web.port() == port {
        return Ok(()); // our own web interface
    }
    bail!("port {port} is in use")
}

async fn web(ctx: &Ctx, action: WebCmd) -> Result<()> {
    use usbnexus_core::api::{self, Request, WebStatusView};
    let call = |req: Request| {
        let socket = ctx.socket.clone();
        async move { api::call::<WebStatusView>(&socket, &req).await }
    };
    let status = match action {
        WebCmd::CheckPort { port } => return check_port(ctx, port),
        WebCmd::HasPassword => {
            if usbnexus_core::daemon::saved_web(&ctx.dir).password_hash.is_some() {
                return Ok(());
            }
            bail!("no web password is set");
        }
        WebCmd::Status => call(Request::WebStatus).await?,
        WebCmd::Disable => {
            call(Request::WebConfigure { enabled: Some(false), lan: None, port: None, password: None }).await?
        }
        WebCmd::Password => {
            let password = tokio::task::spawn_blocking(prompt_new_password).await??;
            let s =
                call(Request::WebConfigure { enabled: None, lan: None, port: None, password: Some(password) }).await?;
            println!("{}", t!("web-password-set"));
            s
        }
        WebCmd::Enable { lan, port } => {
            let current = call(Request::WebStatus).await?;
            let password = if current.password_set {
                None
            } else {
                Some(tokio::task::spawn_blocking(prompt_new_password).await??)
            };
            call(Request::WebConfigure { enabled: Some(true), lan: Some(lan), port, password }).await?
        }
    };
    print_web_status(&status);
    Ok(())
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

async fn policy_cmd(ctx: &Ctx, policy: Option<PolicyArg>) -> Result<()> {
    use usbnexus_core::access::Policy;
    use usbnexus_core::api::{self, Request, StatusView};
    if let Some(p) = policy {
        let policy = match p {
            PolicyArg::Open => Policy::Open,
            PolicyArg::Restricted => Policy::Restricted,
        };
        api::call::<()>(&ctx.socket, &Request::SetPolicy { policy }).await?;
    }
    let status: StatusView = api::call(&ctx.socket, &Request::Status).await?;
    println!(
        "{}",
        match status.policy {
            Policy::Open => t!("policy-open"),
            Policy::Restricted => t!("policy-restricted"),
        }
    );
    Ok(())
}

async fn history(ctx: &Ctx, csv: bool, limit: usize) -> Result<()> {
    use usbnexus_core::api::{self, Request, UsageView};
    use usbnexus_core::usage::{csv_field, format_utc, UsageKind};
    let limit = if csv { None } else { Some(limit) };
    let view: UsageView = api::call(&ctx.socket, &Request::Usage { limit }).await?;
    let kind = |k: UsageKind| match k {
        UsageKind::Paired => t!("history-paired"),
        UsageKind::PairingFailed => t!("history-pairing-failed"),
        UsageKind::Attached => t!("history-attached"),
        UsageKind::Detached => t!("history-detached"),
        UsageKind::Denied => t!("history-denied"),
    };
    let headers = [
        t!("col-time"),
        t!("col-event"),
        t!("col-computer"),
        t!("col-address"),
        t!("col-product"),
        t!("col-device-id"),
        t!("col-duration"),
        t!("col-fingerprint"),
    ];
    let rows: Vec<Vec<String>> = view
        .entries
        .into_iter()
        .map(|e| {
            vec![
                format_utc(e.time),
                kind(e.kind),
                e.computer.unwrap_or_default(),
                e.address.unwrap_or_default(),
                e.device_name.unwrap_or_default(),
                e.device.unwrap_or_default(),
                e.duration_secs.map(|s| s.to_string()).unwrap_or_default(),
                e.fingerprint.unwrap_or_default(),
            ]
        })
        .collect();
    if csv {
        // Oldest first, like a log file.
        let line = |cells: &[String]| cells.iter().map(|c| csv_field(c)).collect::<Vec<_>>().join(",");
        println!("{}", line(&headers));
        for r in rows.iter().rev() {
            println!("{}", line(r));
        }
        return Ok(());
    }
    if rows.is_empty() {
        println!("{}", t!("history-empty"));
        return Ok(());
    }
    println!("{}", t!("history-header", days = view.retention_days));
    let short: Vec<Vec<String>> = rows
        .into_iter()
        .map(|mut r| {
            r.truncate(7);
            r[6] = r[6].parse::<u64>().map(ui::duration).unwrap_or_default();
            r
        })
        .collect();
    ui::table(&headers[..7], &short);
    Ok(())
}
