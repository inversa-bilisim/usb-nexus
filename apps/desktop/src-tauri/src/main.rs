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

#[cfg(unix)]
async fn forward(request: &Request) -> Result<serde_json::Value, ApiError> {
    let socket = usbnexus_core::api::default_socket();
    match usbnexus_core::api::call::<serde_json::Value>(&socket, request).await {
        Ok(v) => Ok(v),
        Err(e) => match e.downcast::<ApiError>() {
            Ok(api) => Err(api),
            // Could not talk to the service at all.
            Err(e) => Err(ApiError::new("service_unavailable", format!("{}: {e:#}", socket.display()))),
        },
    }
}

#[cfg(not(unix))]
async fn forward(_request: &Request) -> Result<serde_json::Value, ApiError> {
    Err(ApiError::new("service_unavailable", "the service transport for this platform is not implemented yet"))
}

#[derive(Serialize)]
struct UiStrings {
    lang: &'static str,
    languages: Vec<(&'static str, &'static str)>,
    messages: BTreeMap<String, String>,
}

/// Interface strings for `lang` (or the system language), as `{name}` templates.
#[tauri::command]
fn ui_strings(lang: Option<String>) -> UiStrings {
    let lang = usbnexus_i18n::detect(lang.as_deref());
    UiStrings { lang, languages: usbnexus_i18n::languages(), messages: usbnexus_i18n::templates(lang) }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![api, ui_strings])
        .run(tauri::generate_context!())
        .expect("error while running USB Nexus");
}
