// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! USB Nexus desktop application.
//!
//! The window is an unprivileged front end: every action is forwarded to the
//! USB Nexus service over its local API socket, which does the privileged
//! work (driver binding, virtual ports) and keeps running when the window is
//! closed.

// Hide the console window on Windows release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::BTreeMap;

use serde::Serialize;
use usbnexus_core::api::{ApiError, Request};

/// Forwards one API request to the service.
#[tauri::command]
async fn api(request: serde_json::Value) -> Result<serde_json::Value, ApiError> {
    let request: Request = serde_json::from_value(request).map_err(|e| ApiError::new("invalid", e.to_string()))?;
    forward(&request).await
}

#[cfg(any(unix, windows))]
async fn forward(request: &Request) -> Result<serde_json::Value, ApiError> {
    let socket = usbnexus_core::api::default_socket();
    match usbnexus_core::api::call::<serde_json::Value>(&socket, request).await {
        Ok(v) => Ok(v),
        Err(e) => match e.downcast::<ApiError>() {
            Ok(api) => Err(api),
            // This user may not use the service's socket (Linux: not in
            // the usbnexus group).
            Err(e)
                if e.chain().any(|c| {
                    c.downcast_ref::<std::io::Error>()
                        .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied)
                }) =>
            {
                Err(ApiError::new("permission_denied", format!("{}: {e:#}", socket.display())))
            }
            // Could not talk to the service at all.
            Err(e) => Err(ApiError::new("service_unavailable", format!("{}: {e:#}", socket.display()))),
        },
    }
}

#[cfg(not(any(unix, windows)))]
async fn forward(_request: &Request) -> Result<serde_json::Value, ApiError> {
    Err(ApiError::new("service_unavailable", "the service transport for this platform is not implemented yet"))
}

/// Linux: lets the current user control the service (asks for the
/// administrator password through polkit), for the "allow access" button.
#[tauri::command]
async fn grant_access() -> Result<(), ApiError> {
    #[cfg(target_os = "linux")]
    {
        let user = std::env::var("USER")
            .or_else(|_| std::env::var("LOGNAME"))
            .map_err(|_| ApiError::new("other", "cannot determine the user name"))?;
        // The service binary sits next to this one (/usr/bin) or on PATH.
        let sibling = std::env::current_exe().ok().and_then(|p| Some(p.parent()?.join("usbnexus")));
        let usbnexus = sibling.filter(|p| p.is_file()).unwrap_or_else(|| std::path::PathBuf::from("usbnexus"));
        let out = tauri::async_runtime::spawn_blocking(move || {
            std::process::Command::new("pkexec").arg(&usbnexus).arg("allow-user").arg(&user).output()
        })
        .await
        .map_err(|e| ApiError::new("other", format!("{e}")))?
        .map_err(|e| ApiError::new("other", format!("running pkexec: {e}")))?;
        if !out.status.success() {
            // 126/127: the password dialog was dismissed.
            let code = out.status.code().unwrap_or(-1);
            let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
            return Err(ApiError::new(if code == 126 || code == 127 { "cancelled" } else { "other" }, stderr));
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(ApiError::new("unsupported", "not needed on this platform"))
    }
}

#[derive(Serialize)]
struct UiStrings {
    /// Operating system (`linux`, `windows`, `macos`), for platform-specific hints.
    os: &'static str,
    lang: &'static str,
    languages: Vec<(&'static str, &'static str)>,
    messages: BTreeMap<String, String>,
}

/// Interface strings for `lang` (or the system language), as `{name}` templates.
#[tauri::command]
fn ui_strings(lang: Option<String>) -> UiStrings {
    let lang = usbnexus_i18n::detect(lang.as_deref());
    UiStrings {
        os: std::env::consts::OS,
        lang,
        languages: usbnexus_i18n::languages(),
        messages: usbnexus_i18n::templates(lang),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api, ui_strings, grant_access])
        .run(tauri::generate_context!())
        .expect("error while running USB Nexus");
}
