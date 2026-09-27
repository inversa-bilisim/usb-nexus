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
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Manager, WindowEvent, Wry};
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use usbnexus_core::api::{ApiError, Request};

const TRAY_ID: &str = "main";
/// Passed by the autostart entry: begin in the notification area.
const HIDDEN_FLAG: &str = "--hidden";

/// Whether the notification area icon could be created; without it,
/// closing the window quits.
struct HasTray(bool);

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

/// The notification area menu in `lang`.
fn tray_menu(app: &AppHandle, lang: &str) -> tauri::Result<Menu<Wry>> {
    let l = usbnexus_i18n::Localizer::new(usbnexus_i18n::detect(Some(lang)));
    let open = MenuItem::with_id(app, "open", l.format("gui-tray-open", None), true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", l.format("gui-tray-quit", None), true, None::<&str>)?;
    Menu::with_items(app, &[&open, &quit])
}

fn show_window(app: &AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.show();
        let _ = w.unminimize();
        let _ = w.set_focus();
    }
}

/// The interface changed its language: translate the tray menu too.
#[tauri::command]
fn set_language(app: AppHandle, lang: String) {
    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        if let Ok(menu) = tray_menu(&app, &lang) {
            let _ = tray.set_menu(Some(menu));
        }
    }
}

/// Whether the app starts when the user signs in (`None`: not supported here).
#[tauri::command]
fn autostart_get(app: AppHandle) -> Option<bool> {
    app.autolaunch().is_enabled().ok()
}

#[tauri::command]
fn autostart_set(app: AppHandle, enabled: bool) -> Result<(), ApiError> {
    let launch = app.autolaunch();
    if enabled { launch.enable() } else { launch.disable() }.map_err(|e| ApiError::new("other", e.to_string()))
}

fn main() {
    let start_hidden = std::env::args().any(|a| a == HIDDEN_FLAG);
    tauri::Builder::default()
        // A second launch (shortcut, autostart) brings the window back instead.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| show_window(app)))
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec![HIDDEN_FLAG])))
        .invoke_handler(tauri::generate_handler![
            api,
            ui_strings,
            grant_access,
            set_language,
            autostart_get,
            autostart_set
        ])
        .setup(move |app| {
            let handle = app.handle().clone();
            let tray = TrayIconBuilder::with_id(TRAY_ID)
                .icon(app.default_window_icon().cloned().expect("the app has an icon"))
                .tooltip("USB Nexus")
                .menu(&tray_menu(&handle, usbnexus_i18n::detect(None))?)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "open" => show_window(app),
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left, button_state: MouseButtonState::Up, ..
                    } = event
                    {
                        show_window(tray.app_handle());
                    }
                })
                .build(app);
            let has_tray = match tray {
                Ok(_) => true,
                Err(e) => {
                    eprintln!("no notification area icon: {e}");
                    false
                }
            };
            app.manage(HasTray(has_tray));
            if !(start_hidden && has_tray) {
                show_window(&handle);
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // Closing the window keeps the app in the notification area; the
            // service does the real work anyway.
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.state::<HasTray>().0 {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running USB Nexus");
}
