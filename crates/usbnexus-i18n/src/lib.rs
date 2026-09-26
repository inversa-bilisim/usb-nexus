// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! User interface translations.
//!
//! Strings live in Fluent files under the repository's `locales/` directory
//! so the same files can later be loaded by the graphical interface. They are
//! embedded at build time. English is the fallback for anything missing.
//!
//! Adding a language: create `locales/<code>.ftl` with every message from
//! `en.ftl` and add it to [`LOCALES`].

use std::borrow::Cow;
use std::sync::OnceLock;

use fluent_bundle::concurrent::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use unic_langid::LanguageIdentifier;

/// Embedded locales as `(language code, native name, Fluent source)`.
pub const LOCALES: &[(&str, &str, &str)] = &[
    ("en", "English", include_str!("../../../locales/en.ftl")),
    ("tr", "Türkçe", include_str!("../../../locales/tr.ftl")),
];

pub const FALLBACK: &str = "en";

pub struct Localizer {
    lang: &'static str,
    /// Selected language first, then the fallback.
    bundles: Vec<FluentBundle<FluentResource>>,
}

fn bundle(code: &str, source: &str) -> FluentBundle<FluentResource> {
    let id: LanguageIdentifier = code.parse().expect("locale codes are valid");
    let res = FluentResource::try_new(source.to_string()).unwrap_or_else(|(_, e)| panic!("invalid {code}.ftl: {e:?}"));
    let mut b = FluentBundle::new_concurrent(vec![id]);
    // Unicode isolation marks show up as garbage in many terminals.
    b.set_use_isolating(false);
    b.add_resource(res).unwrap_or_else(|e| panic!("duplicate messages in {code}.ftl: {e:?}"));
    b
}

fn source(code: &str) -> Option<(&'static str, &'static str)> {
    LOCALES.iter().find(|(c, _, _)| *c == code).map(|(c, _, s)| (*c, *s))
}

/// Maps a locale string such as `tr_TR.UTF-8`, `tr-TR` or `TR` to a supported code.
pub fn match_locale(spec: &str) -> Option<&'static str> {
    let lang = spec.split(['.', '@']).next()?.split(['_', '-']).next()?.to_ascii_lowercase();
    LOCALES.iter().map(|(c, _, _)| *c).find(|c| *c == lang)
}

/// Picks a language: explicit choice, then `LC_ALL`, `LC_MESSAGES`, `LANG`,
/// `LANGUAGE`, then the operating system's preferred UI languages (on
/// Windows and macOS these variables are usually unset), then the fallback.
pub fn detect(explicit: Option<&str>) -> &'static str {
    if let Some(code) = explicit.and_then(match_locale) {
        return code;
    }
    for var in ["LC_ALL", "LC_MESSAGES", "LANG", "LANGUAGE"] {
        if let Ok(v) = std::env::var(var) {
            // LANGUAGE may hold a priority list such as "tr:en".
            if let Some(code) = v.split(':').find_map(match_locale) {
                return code;
            }
        }
    }
    // In order of preference, e.g. ["fr-FR", "tr-TR", "en-US"]: the first
    // one we have wins.
    sys_locale::get_locales().find_map(|l| match_locale(&l)).unwrap_or(FALLBACK)
}

impl Localizer {
    pub fn new(code: &str) -> Self {
        let (lang, src) = source(code).or_else(|| source(FALLBACK)).expect("fallback locale exists");
        let mut bundles = vec![bundle(lang, src)];
        if lang != FALLBACK {
            let (c, s) = source(FALLBACK).expect("fallback locale exists");
            bundles.push(bundle(c, s));
        }
        Localizer { lang, bundles }
    }

    pub fn lang(&self) -> &'static str {
        self.lang
    }

    /// Whether any loaded locale defines `id`.
    pub fn has(&self, id: &str) -> bool {
        self.bundles.iter().any(|b| b.has_message(id))
    }

    /// Formats a message; returns the id itself when no locale has it.
    pub fn format(&self, id: &str, args: Option<&FluentArgs>) -> String {
        for b in &self.bundles {
            if let Some(pattern) = b.get_message(id).and_then(|m| m.value()) {
                let mut errors = vec![];
                return b.format_pattern(pattern, args, &mut errors).into_owned();
            }
        }
        id.to_string()
    }
}

/// Converts a simple Fluent pattern to a template with `{name}` placeholders.
/// Only text and variable references are supported, which is all the UI uses.
fn pattern_template(p: &fluent_syntax::ast::Pattern<&str>) -> String {
    use fluent_syntax::ast::{Expression, InlineExpression, PatternElement};
    let mut out = String::new();
    for el in &p.elements {
        match el {
            PatternElement::TextElement { value } => out.push_str(value),
            PatternElement::Placeable {
                expression: Expression::Inline(InlineExpression::VariableReference { id }),
            } => {
                out.push('{');
                out.push_str(id.name);
                out.push('}');
            }
            PatternElement::Placeable { .. } => {}
        }
    }
    out
}

/// All messages of a language as `{name}` templates, with English filling
/// any gaps. Used by the graphical interface.
pub fn templates(code: &str) -> std::collections::BTreeMap<String, String> {
    let mut out = std::collections::BTreeMap::new();
    let chosen = source(code).map(|(c, _)| c).unwrap_or(FALLBACK);
    for c in [FALLBACK, chosen] {
        let (_, src) = source(c).expect("known locale");
        let res = fluent_syntax::parser::parse(src).unwrap_or_else(|(r, _)| r);
        for entry in &res.body {
            if let fluent_syntax::ast::Entry::Message(m) = entry {
                if let Some(v) = &m.value {
                    out.insert(m.id.name.to_string(), pattern_template(v));
                }
            }
        }
    }
    out
}

/// NSIS language of each locale, as named in the Windows installer's
/// `languages` list (`packaging/windows/tauri.bundle.json`).
const NSIS_LANGUAGES: &[(&str, &str)] = &[("en", "English"), ("tr", "Turkish")];

/// The Windows installer's own texts (`setup-*` messages) as NSIS
/// `LangString`s for every language; `packaging/windows/usbnexus-strings.nsh` holds
/// the output (see the `nsis_strings_are_current` test).
pub fn nsis_strings() -> String {
    // NSIS reads source files without a byte order mark as ANSI.
    let mut out = String::from(
        "\u{feff}; Generated from locales/*.ftl (setup-* messages); do not edit.\n\
         ; Regenerate: UPDATE_NSIS_STRINGS=1 cargo test -p usbnexus-i18n\n",
    );
    for (code, _, _) in LOCALES {
        let nsis = NSIS_LANGUAGES.iter().find(|(c, _)| c == code).map(|(_, n)| *n);
        let Some(nsis) = nsis else { continue };
        out.push('\n');
        for (id, text) in templates(code).into_iter().filter(|(id, _)| id.starts_with("setup-")) {
            let text = text.replace('$', "$$").replace('"', "$\\\"").replace('\n', "$\\r$\\n");
            out.push_str(&format!(
                "LangString {} ${{LANG_{}}} \"{}\"\n",
                id.replace('-', "_"),
                nsis.to_uppercase(),
                text
            ));
        }
    }
    out
}

/// Supported languages as `(code, native name)`.
pub fn languages() -> Vec<(&'static str, &'static str)> {
    LOCALES.iter().map(|(c, n, _)| (*c, *n)).collect()
}

static GLOBAL: OnceLock<Localizer> = OnceLock::new();

/// Sets the process-wide language. Only the first call has an effect.
pub fn init(code: &str) -> &'static Localizer {
    GLOBAL.get_or_init(|| Localizer::new(code))
}

/// The process-wide localizer (initialised from the environment on first use).
pub fn global() -> &'static Localizer {
    GLOBAL.get_or_init(|| Localizer::new(detect(None)))
}

/// Builds Fluent arguments; used by the [`t!`] macro.
pub fn args<'a>(pairs: &[(&'a str, FluentValue<'a>)]) -> FluentArgs<'a> {
    let mut a = FluentArgs::new();
    for (k, v) in pairs {
        a.set(Cow::Borrowed(*k), v.clone());
    }
    a
}

pub use fluent_bundle::FluentValue as Value;

/// Translates a message: `t!("pair-ok", name = n, fp = f)`.
#[macro_export]
macro_rules! t {
    ($id:expr) => {
        $crate::global().format($id, None)
    };
    ($id:expr, $($key:ident = $val:expr),+ $(,)?) => {
        $crate::global().format(
            $id,
            Some(&$crate::args(&[$((stringify!($key), $crate::Value::from($val))),+])),
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    fn ids(src: &str) -> BTreeSet<String> {
        let res = FluentResource::try_new(src.to_string()).expect("parses");
        res.entries()
            .filter_map(|e| match e {
                fluent_syntax::ast::Entry::Message(m) => Some(m.id.name.to_string()),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn every_locale_is_complete() {
        let reference = ids(source(FALLBACK).unwrap().1);
        assert!(!reference.is_empty());
        for (code, _, src) in LOCALES {
            let got = ids(src);
            let missing: Vec<_> = reference.difference(&got).collect();
            let extra: Vec<_> = got.difference(&reference).collect();
            assert!(missing.is_empty(), "{code}.ftl is missing: {missing:?}");
            assert!(extra.is_empty(), "{code}.ftl has unknown messages: {extra:?}");
            // Building the bundle panics on syntax or duplicate errors.
            bundle(code, src);
        }
    }

    #[test]
    fn formats_with_arguments() {
        let tr = Localizer::new("tr");
        assert_eq!(tr.format("pin-show", Some(&args(&[("pin", "123456".into())]))), "Eşleştirme PIN kodu: 123456");
        let en = Localizer::new("en");
        assert_eq!(en.format("attach-retrying", Some(&args(&[("seconds", 3.into())]))), "Reconnecting in 3 seconds…");
        assert_eq!(en.format("no-such-message", None), "no-such-message");
    }

    #[test]
    fn templates_for_ui() {
        let t = templates("tr");
        assert_eq!(t["pin-show"], "Eşleştirme PIN kodu: {pin}");
        assert_eq!(templates("xx")["pin-show"], "Pairing PIN: {pin}");
        assert_eq!(t.len(), templates("en").len());
    }

    fn repo_file(path: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..").join(path)
    }

    #[test]
    fn nsis_strings_are_current() {
        let path = repo_file("packaging/windows/usbnexus-strings.nsh");
        let want = nsis_strings();
        if std::env::var_os("UPDATE_NSIS_STRINGS").is_some() {
            std::fs::write(&path, &want).unwrap();
        }
        // Windows checkouts may turn line endings into CRLF.
        let have = std::fs::read_to_string(&path).unwrap_or_default().replace("\r\n", "\n");
        assert!(
            have == want,
            "{} is out of date; run: UPDATE_NSIS_STRINGS=1 cargo test -p usbnexus-i18n",
            path.display()
        );
    }

    #[test]
    fn every_locale_is_in_the_installer() {
        let bundle = std::fs::read_to_string(repo_file("packaging/windows/tauri.bundle.json")).unwrap();
        for (code, _, _) in LOCALES {
            let (_, nsis) = NSIS_LANGUAGES.iter().find(|(c, _)| c == code).unwrap_or_else(|| {
                panic!("add the NSIS language of {code} to NSIS_LANGUAGES");
            });
            assert!(bundle.contains(&format!("\"{nsis}\"")), "add {nsis} to the languages in tauri.bundle.json");
        }
        for (id, text) in templates("en").into_iter().filter(|(id, _)| id.starts_with("setup-")) {
            assert!(!text.contains('{'), "{id}: installer texts cannot have variables");
        }
    }

    #[test]
    fn unknown_language_falls_back() {
        assert_eq!(Localizer::new("xx").lang(), "en");
    }

    #[test]
    fn locale_matching() {
        assert_eq!(match_locale("tr_TR.UTF-8"), Some("tr"));
        assert_eq!(match_locale("TR"), Some("tr"));
        assert_eq!(match_locale("en-US"), Some("en"));
        assert_eq!(match_locale("de_DE"), None);
        assert_eq!(detect(Some("tr")), "tr");
    }
}
