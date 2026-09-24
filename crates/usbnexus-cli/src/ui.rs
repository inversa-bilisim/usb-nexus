// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Terminal presentation: localized help, tables and error messages.

use clap::{Arg, ArgAction, Command};
use usbnexus_core::api::ApiError;
use usbnexus_core::client::ClientError;
use usbnexus_core::control::{ErrorCode, RemoteError};
use usbnexus_i18n::{global, t};

/// Replaces clap's built-in English help texts with translations.
///
/// Conventions: an argument with id `foo_bar` uses message `arg-foo-bar`;
/// a subcommand `baz` uses `cmd-baz-about`.
pub fn localize(cmd: Command, is_root: bool) -> Command {
    let l = global();
    let template =
        format!("{{about-with-newline}}\n{} {{usage}}\n\n{{all-args}}{{after-help}}", l.format("help-usage", None));
    let mut cmd = cmd
        .help_template(template)
        .subcommand_help_heading(l.format("help-commands", None))
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .action(ArgAction::Help)
                .global(is_root)
                .help(l.format("arg-help", None)),
        );
    if is_root {
        cmd = cmd
            .about(t!("app-about"))
            .disable_version_flag(true)
            .arg(Arg::new("version").short('V').long("version").action(ArgAction::Version).help(t!("arg-version")));
    } else {
        let key = format!("cmd-{}-about", cmd.get_name());
        if l.has(&key) {
            cmd = cmd.about(l.format(&key, None));
        }
    }
    let options = l.format("help-options", None);
    let arguments = l.format("help-arguments", None);
    cmd = cmd.mut_args(|a| {
        let key = format!("arg-{}", a.get_id().as_str().replace('_', "-"));
        let heading = if a.is_positional() { arguments.clone() } else { options.clone() };
        let a = a.help_heading(heading);
        if l.has(&key) {
            a.help(l.format(&key, None))
        } else {
            a
        }
    });
    let names: Vec<String> = cmd.get_subcommands().map(|s| s.get_name().to_string()).collect();
    for name in names {
        cmd = cmd.mut_subcommand(name, |s| localize(s, false));
    }
    cmd
}

/// Prints rows as left-aligned columns.
pub fn table(headers: &[String], rows: &[Vec<String>]) {
    let width = |s: &str| s.chars().count();
    let mut w: Vec<usize> = headers.iter().map(|h| width(h)).collect();
    for r in rows {
        for (i, c) in r.iter().enumerate() {
            w[i] = w[i].max(width(c));
        }
    }
    let line = |cells: &[String]| {
        let mut out = String::new();
        for (i, c) in cells.iter().enumerate() {
            if i + 1 == cells.len() {
                out.push_str(c);
            } else {
                out.push_str(c);
                out.push_str(&" ".repeat(w[i] - width(c) + 2));
            }
        }
        println!("  {}", out.trim_end());
    };
    line(headers);
    for r in rows {
        line(r);
    }
}

fn remote_key(code: ErrorCode) -> &'static str {
    match code {
        ErrorCode::Version => "err-version",
        ErrorCode::NotTrusted => "err-not-trusted",
        ErrorCode::PairingClosed => "err-pairing-closed",
        ErrorCode::PairingFailed => "err-pairing-failed",
        ErrorCode::NoSuchDevice => "err-no-such-device",
        ErrorCode::DeviceBusy => "err-device-busy",
        ErrorCode::Internal => "err-internal",
        ErrorCode::Protocol => "err-protocol",
    }
}

fn permission_denied(e: &anyhow::Error) -> bool {
    e.chain()
        .any(|c| c.downcast_ref::<std::io::Error>().is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied))
}

/// Turns an error into a localized, user-facing message.
pub fn describe(e: &anyhow::Error, target: Option<&str>) -> String {
    if let Some(c) = e.downcast_ref::<ClientError>() {
        return match c {
            ClientError::PairingRequired { name, .. } => {
                t!("err-pairing-required", name = name.as_str(), target = target.unwrap_or(name))
            }
            ClientError::NotFound(_) => t!("err-not-found"),
        };
    }
    if let Some(r) = e.downcast_ref::<RemoteError>() {
        return t!(remote_key(r.code));
    }
    if let Some(a) = e.downcast_ref::<ApiError>() {
        let key = format!("err-{}", a.code.replace('_', "-"));
        if a.code != "other" && a.code != "pairing_required" && global().has(&key) {
            return global().format(&key, None);
        }
    }
    let mut msg = t!("error-prefix", detail = format!("{e:#}"));
    if permission_denied(e) {
        msg.push('\n');
        msg.push_str(&t!("hint-root"));
    }
    msg
}
