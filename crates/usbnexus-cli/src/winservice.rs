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

use anyhow::{Context, Result};
use usbnexus_i18n::t;
use windows_service::service::{
    ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode, ServiceInfo,
    ServiceStartType, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::{define_windows_service, service_dispatcher};

use crate::{Ctx, ServeOpts};

pub const SERVICE_NAME: &str = "usbnexus";
const DISPLAY_NAME: &str = "USB Nexus";
const DESCRIPTION: &str = "Shares USB devices over the network with encryption, pairing and automatic reconnection.";
const FIREWALL_RULE: &str = "USB Nexus";

/// Context handed from `main` to the service entry point.
static CONTEXT: OnceLock<Ctx> = OnceLock::new();

pub fn install() -> Result<()> {
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
    let service = manager
        .create_service(&info, ServiceAccess::CHANGE_CONFIG | ServiceAccess::START)
        .context("creating the service")?;
    service.set_description(DESCRIPTION)?;

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
    println!("{}", t!("service-installed"));
    Ok(())
}

pub fn uninstall() -> Result<()> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
        .context("opening the service manager (run as administrator)")?;
    let service = manager
        .open_service(SERVICE_NAME, ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE)
        .context("opening the service")?;
    if service.query_status()?.current_state != ServiceState::Stopped {
        let _ = service.stop();
        for _ in 0..50 {
            if service.query_status()?.current_state == ServiceState::Stopped {
                break;
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    }
    service.delete().context("deleting the service")?;
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
