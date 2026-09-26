// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Running the USB Nexus service as a Windows service.
//!
//! `usbnexus service install` registers the current executable to start
//! automatically as LocalSystem with the arguments `service run`, and adds a
//! firewall rule so other computers and LAN discovery can reach it.

use std::ffi::{OsStr, OsString};
use std::sync::OnceLock;
use std::time::Duration;

use anyhow::{bail, Context, Result};
use usbnexus_i18n::t;
use windows_service::service::{
    ServiceAccess, ServiceAction, ServiceActionType, ServiceControl, ServiceControlAccept, ServiceErrorControl,
    ServiceExitCode, ServiceFailureActions, ServiceFailureResetPeriod, ServiceInfo, ServiceStartType, ServiceState,
    ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::{define_windows_service, service_dispatcher};

use crate::{Ctx, InstallOpts, ServeOpts, WebAccessArg};

pub const SERVICE_NAME: &str = "usbnexus";
const DISPLAY_NAME: &str = "USB Nexus";
const DESCRIPTION: &str = "Shares USB devices over the network with encryption, pairing and automatic reconnection.";
const FIREWALL_RULE: &str = "USB Nexus";

/// Context handed from `main` to the service entry point.
static CONTEXT: OnceLock<Ctx> = OnceLock::new();

/// Kernel service of the VirtualBox USB capture monitor.
const MONITOR_SERVICE: &str = "VBoxUSBMon";

/// Bundled VBoxUSB drivers for this CPU: `<exe dir>\drivers\<arch>`.
fn driver_dir() -> Option<std::path::PathBuf> {
    let arch = match std::env::consts::ARCH {
        "x86_64" => "x64",
        "aarch64" => "arm64",
        _ => return None,
    };
    let dir = std::env::current_exe().ok()?.parent()?.join("drivers").join(arch);
    dir.join("VBoxUSBMon.sys").is_file().then_some(dir)
}

/// Adds VBoxUSB to the driver store and registers the VBoxUSBMon kernel
/// service, unless another program (e.g. usbipd-win) already did.
fn install_drivers(manager: &ServiceManager) -> Result<()> {
    let Some(dir) = driver_dir() else {
        tracing::warn!("bundled VBoxUSB drivers not found; sharing local devices will be unavailable");
        return Ok(());
    };
    let inf = dir.join("VBoxUSB.inf");
    let out = std::process::Command::new("pnputil").arg("/add-driver").arg(&inf).output().context("running pnputil")?;
    if !out.status.success() {
        bail!("pnputil /add-driver failed: {}", String::from_utf8_lossy(&out.stdout).trim());
    }
    if manager.open_service(MONITOR_SERVICE, ServiceAccess::QUERY_STATUS).is_ok() {
        return Ok(());
    }
    let info = ServiceInfo {
        name: OsString::from(MONITOR_SERVICE),
        display_name: OsString::from("VirtualBox USB Monitor Service"),
        service_type: ServiceType::KERNEL_DRIVER,
        start_type: ServiceStartType::OnDemand,
        error_control: ServiceErrorControl::Normal,
        executable_path: dir.join("VBoxUSBMon.sys"),
        launch_arguments: vec![],
        dependencies: vec![],
        account_name: None,
        account_password: None,
    };
    manager.create_service(&info, ServiceAccess::QUERY_STATUS).context("registering VBoxUSBMon")?;
    Ok(())
}

/// Removes the VBoxUSBMon service if it is the one we registered.
fn uninstall_drivers(manager: &ServiceManager) {
    let Some(dir) = driver_dir() else { return };
    let Ok(service) = manager.open_service(
        MONITOR_SERVICE,
        ServiceAccess::QUERY_CONFIG | ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE,
    ) else {
        return;
    };
    let ours = service
        .query_config()
        .map(|c| c.executable_path.to_string_lossy().to_lowercase().contains(&dir.to_string_lossy().to_lowercase()))
        .unwrap_or(false);
    if ours {
        let _ = service.stop();
        let _ = service.delete();
    }
}

/// Starts the capture monitor so devices can be shared (best effort).
fn start_monitor() {
    let Ok(manager) = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT) else { return };
    if let Ok(service) = manager.open_service(MONITOR_SERVICE, ServiceAccess::QUERY_STATUS | ServiceAccess::START) {
        if service.query_status().map(|s| s.current_state != ServiceState::Running).unwrap_or(false) {
            if let Err(e) = service.start(&[] as &[&OsStr]) {
                tracing::warn!("could not start VBoxUSBMon: {e}");
            }
        }
    }
}

/// Installs (or updates) and starts the service with the roles and web
/// settings in `opts`; returns once the service answers.
pub async fn install(ctx: &Ctx, opts: InstallOpts) -> Result<()> {
    use usbnexus_core::api::Roles;
    use usbnexus_core::daemon::{apply_setup, Setup};

    let roles = Roles { server: !opts.no_server, client: !opts.no_client };
    let web_password = match &opts.web_password_file {
        Some(path) => Some(read_password_file(path)?),
        None => None,
    };
    let setup = Setup {
        roles: Some(roles),
        web_enabled: opts.web.map(|w| !matches!(w, WebAccessArg::Off)),
        web_lan: match opts.web {
            Some(WebAccessArg::Network) => Some(true),
            Some(WebAccessArg::Local) => Some(false),
            _ => None,
        },
        web_port: opts.web_port,
        web_password,
    };
    let exe = std::env::current_exe()?;
    let manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE,
    )
    .context("opening the service manager (run as administrator)")?;
    let info = ServiceInfo {
        name: OsString::from(SERVICE_NAME),
        display_name: OsString::from(DISPLAY_NAME),
        service_type: ServiceType::OWN_PROCESS,
        start_type: ServiceStartType::AutoStart,
        error_control: ServiceErrorControl::Normal,
        executable_path: exe.clone(),
        launch_arguments: vec![OsString::from("service"), OsString::from("run")],
        dependencies: vec![],
        account_name: None, // LocalSystem
        account_password: None,
    };
    // Installing over an earlier version updates the existing service; it
    // is stopped first, as it rewrites the configuration while running.
    let access =
        ServiceAccess::CHANGE_CONFIG | ServiceAccess::START | ServiceAccess::STOP | ServiceAccess::QUERY_STATUS;
    let existing = manager.open_service(SERVICE_NAME, access).ok();
    if let Some(service) = &existing {
        stop_and_wait(service)?;
    }
    apply_setup(&ctx.dir, setup)?;
    if roles.server {
        install_drivers(&manager)?;
    }
    let service = match existing {
        Some(service) => {
            service.change_config(&info).context("updating the service")?;
            service
        }
        None => manager.create_service(&info, access).context("creating the service")?,
    };
    service.set_description(DESCRIPTION)?;
    // Restart after a crash or an error exit: 5 s, 10 s, then every 30 s;
    // the count resets after a day without failures.
    let restart = |secs| ServiceAction { action_type: ServiceActionType::Restart, delay: Duration::from_secs(secs) };
    service
        .update_failure_actions(ServiceFailureActions {
            reset_period: ServiceFailureResetPeriod::After(Duration::from_secs(24 * 60 * 60)),
            reboot_msg: None,
            command: None,
            actions: Some(vec![restart(5), restart(10), restart(30)]),
        })
        .context("setting the service recovery actions")?;
    service.set_failure_actions_on_non_crash_failures(true)?;

    // Best effort: without it only outgoing connections work.
    let program = format!("program={}", exe.display());
    let _ = std::process::Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            &format!("name={FIREWALL_RULE}"),
            "dir=in",
            "action=allow",
            &program,
        ])
        .output();

    service.start(&[] as &[&OsStr]).context("starting the service")?;
    wait_until_answering(ctx).await;
    println!("{}", t!("service-installed"));
    Ok(())
}

/// Reads a password file: UTF-8, or UTF-16LE with a byte order mark (as
/// the installer writes it); a trailing line break is ignored.
fn read_password_file(path: &std::path::Path) -> Result<String> {
    let data = std::fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let text = match data.strip_prefix(&[0xFF, 0xFE]) {
        Some(utf16) => {
            let units: Vec<u16> = utf16.chunks_exact(2).map(|c| u16::from_le_bytes([c[0], c[1]])).collect();
            String::from_utf16(&units).context("password file is not valid UTF-16")?
        }
        None => String::from_utf8(data).context("password file is not valid UTF-8")?,
    };
    Ok(text.trim_start_matches('\u{feff}').trim_end_matches(['\r', '\n']).to_string())
}

/// Waits (up to 30 s) until the started service answers on its pipe, so
/// whatever runs next (the app, the installer's browser) finds it ready.
async fn wait_until_answering(ctx: &Ctx) {
    use usbnexus_core::api::{self, Request, StatusView};
    for _ in 0..60 {
        if api::call::<StatusView>(&ctx.socket, &Request::Status).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    tracing::warn!("the service did not answer within 30 s");
}

/// Stops a running service and waits (up to 5 s) until it has stopped.
fn stop_and_wait(service: &windows_service::service::Service) -> Result<()> {
    if service.query_status()?.current_state != ServiceState::Stopped {
        let _ = service.stop();
        for _ in 0..50 {
            if service.query_status()?.current_state == ServiceState::Stopped {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("opening the service manager (run as administrator)")?;
    let service = manager
        .open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)
        .context("opening the service")?;
    stop_and_wait(&service)?;
    service.delete().context("deleting the service")?;
    uninstall_drivers(&manager);
    let _ = std::process::Command::new("netsh")
        .args(["advfirewall", "firewall", "delete", "rule", &format!("name={FIREWALL_RULE}")])
        .output();
    println!("{}", t!("service-removed"));
    Ok(())
}

/// Entry point used by the service manager; blocks until the service stops.
pub fn run(ctx: Ctx) -> Result<()> {
    let _ = CONTEXT.set(ctx);
    service_dispatcher::start(SERVICE_NAME, ffi_service_main).context("starting the service dispatcher")
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_args: Vec<OsString>) {
    if let Err(e) = run_service() {
        tracing::error!("service failed: {e:#}");
    }
}

fn status(state: ServiceState, code: u32) -> ServiceStatus {
    ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: state,
        controls_accepted: if state == ServiceState::Running {
            ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN
        } else {
            ServiceControlAccept::empty()
        },
        exit_code: ServiceExitCode::Win32(code),
        checkpoint: 0,
        wait_hint: Duration::from_secs(10),
        process_id: None,
    }
}

fn run_service() -> Result<()> {
    let ctx = CONTEXT.get().context("service context missing")?;
    let (stop_tx, stop_rx) = tokio::sync::watch::channel(false);
    let handle = service_control_handler::register(SERVICE_NAME, move |control| match control {
        ServiceControl::Stop | ServiceControl::Shutdown => {
            let _ = stop_tx.send(true);
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    })?;
    handle.set_service_status(status(ServiceState::StartPending, 0))?;

    if usbnexus_core::daemon::saved_roles(&ctx.dir).server {
        start_monitor();
    }
    let rt = tokio::runtime::Runtime::new()?;
    let result = rt.block_on(async {
        let mut stop_rx = stop_rx;
        let stop = async move {
            let _ = stop_rx.wait_for(|stop| *stop).await;
        };
        handle.set_service_status(status(ServiceState::Running, 0))?;
        crate::run_daemon(ctx, ServeOpts::service_defaults(), stop).await
    });
    if let Err(e) = &result {
        tracing::error!("service stopped with an error: {e:#}");
    }
    handle.set_service_status(status(ServiceState::Stopped, if result.is_ok() { 0 } else { 1 }))?;
    result
}
