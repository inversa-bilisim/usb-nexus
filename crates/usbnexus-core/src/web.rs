// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Web interface: the desktop app's pages, served by the service over HTTPS.
//!
//! Security model:
//! * Off by default; when on, it listens on localhost only unless LAN access
//!   is enabled explicitly.
//! * Every page but the login needs a session, obtained with the web
//!   password (stored as an Argon2 hash). Failed logins are rate limited
//!   per client address.
//! * The session cookie is `HttpOnly; Secure; SameSite=Strict`, and API
//!   calls must carry an `X-USBNexus` header, which other sites cannot add
//!   without a CORS preflight that is never granted.
//! * A strict Content-Security-Policy forbids inline script and framing.
//! * Reconfiguring the web interface itself is only possible from the
//!   local CLI, not from a browser.

use std::collections::HashMap;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use axum::extract::{Query, State};
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use hyper_util::rt::TokioIo;
use hyper_util::service::TowerToHyperService;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::task::JoinHandle;
use tokio_rustls::TlsAcceptor;

use crate::api::{ApiError, Request, Response as ApiResponse};
use crate::daemon::Daemon;
use crate::identity::{fingerprint, Identity};

pub const DEFAULT_PORT: u16 = 3242;

const SESSION_COOKIE: &str = "usbnexus_session";
const SESSION_LIFETIME: Duration = Duration::from_secs(12 * 3600);
const MAX_FAILURES: u32 = 5;
const LOCKOUT: Duration = Duration::from_secs(60);
const CSRF_HEADER: &str = "x-usbnexus";
const MIN_PASSWORD_LEN: usize = 8;

const CSP: &str = "default-src 'self'; script-src 'self'; style-src 'self'; img-src 'self' data:; \
                   connect-src 'self'; frame-ancestors 'none'; base-uri 'none'; form-action 'self'";

// The same files as the desktop app, plus the browser shim.
const INDEX_HTML: &str = include_str!("../../../apps/desktop/ui/index.html");
const APP_JS: &str = include_str!("../../../apps/desktop/ui/app.js");
const WEB_JS: &str = include_str!("../../../apps/desktop/ui/web.js");
const STYLE_CSS: &str = include_str!("../../../apps/desktop/ui/style.css");
const LOGO_SVG: &str = include_str!("../../../apps/desktop/ui/logo.svg");

/// Persisted web settings (part of the service configuration).
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebSettings {
    #[serde(default)]
    pub enabled: bool,
    /// Listen on all interfaces instead of localhost only.
    #[serde(default)]
    pub lan: bool,
    #[serde(default)]
    pub port: Option<u16>,
    /// Argon2 PHC string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
}

impl WebSettings {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(DEFAULT_PORT)
    }

    pub fn listen_addr(&self) -> SocketAddr {
        let ip: IpAddr = if self.lan { [0, 0, 0, 0].into() } else { [127, 0, 0, 1].into() };
        SocketAddr::new(ip, self.port())
    }
}

/// Hashes a new web password.
pub fn hash_password(password: &str) -> Result<String> {
    use argon2::password_hash::{PasswordHasher, SaltString};
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(ApiError::new("weak_password", format!("at least {MIN_PASSWORD_LEN} characters")).into());
    }
    let salt = SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let hash = argon2::Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("hashing password: {e}"))?;
    Ok(hash.to_string())
}

fn verify_password(hash: &str, password: &str) -> bool {
    use argon2::password_hash::{PasswordHash, PasswordVerifier};
    PasswordHash::new(hash)
        .map(|h| argon2::Argon2::default().verify_password(password.as_bytes(), &h).is_ok())
        .unwrap_or(false)
}

/// Loads or creates the HTTPS certificate of the web interface. Unlike the
/// tunnel identity it names this computer, so browsers can match it.
pub fn web_identity(state_dir: &Path, hostname: &str) -> Result<Identity> {
    let cert_path = state_dir.join("web-cert.der");
    let key_path = state_dir.join("web-key.der");
    if let (Ok(cert_der), Ok(key_der)) = (std::fs::read(&cert_path), std::fs::read(&key_path)) {
        return Ok(Identity { cert_der, key_der });
    }
    let label: String = hostname.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
    let mut names = vec!["localhost".to_string(), "127.0.0.1".to_string(), "::1".to_string()];
    if !label.is_empty() {
        names.push(label.clone());
        names.push(format!("{label}.local"));
    }
    let key = rcgen::KeyPair::generate().context("generating web key")?;
    let mut params = rcgen::CertificateParams::new(names).context("web certificate parameters")?;
    params.distinguished_name.push(rcgen::DnType::CommonName, format!("USB Nexus ({hostname})"));
    let cert = params.self_signed(&key).context("signing web certificate")?;
    let id = Identity { cert_der: cert.der().to_vec(), key_der: key.serialize_der() };
    crate::identity::write_private(&key_path, &id.key_der)?;
    std::fs::write(&cert_path, &id.cert_der)?;
    Ok(id)
}

fn tls_config(id: &Identity) -> Result<Arc<rustls::ServerConfig>> {
    use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
    let provider = Arc::new(rustls::crypto::ring::default_provider());
    let mut cfg = rustls::ServerConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])?
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(id.cert_der.clone())],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(id.key_der.clone())),
        )
        .context("loading web certificate")?;
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    Ok(Arc::new(cfg))
}

struct WebState {
    daemon: Daemon,
    name: String,
    password_hash: String,
    /// Session token -> expiry.
    sessions: Mutex<HashMap<String, Instant>>,
    /// Client address -> (consecutive failures, time of last failure).
    failures: Mutex<HashMap<IpAddr, (u32, Instant)>>,
}

#[derive(Clone, Copy)]
struct Peer(SocketAddr);

/// A running web interface; stops when dropped.
pub struct WebServer {
    task: Option<JoinHandle<()>>,
    pub addr: SocketAddr,
    pub fingerprint: String,
}

impl WebServer {
    /// Stops the server and waits until its listening socket is closed, so
    /// the port can be bound again right away.
    pub async fn stop(mut self) {
        if let Some(task) = self.task.take() {
            task.abort();
            let _ = task.await;
        }
    }
}

impl Drop for WebServer {
    fn drop(&mut self) {
        if let Some(task) = &self.task {
            task.abort();
        }
    }
}

/// Starts the web interface for `daemon`.
pub async fn start(daemon: Daemon, name: &str, id: &Identity, settings: &WebSettings) -> Result<WebServer> {
    let password_hash = settings
        .password_hash
        .clone()
        .ok_or_else(|| anyhow::Error::from(ApiError::new("password_required", "set a web password first")))?;
    let listener = TcpListener::bind(settings.listen_addr())
        .await
        .with_context(|| format!("listening on {}", settings.listen_addr()))?;
    let addr = listener.local_addr()?;
    let acceptor = TlsAcceptor::from(tls_config(id)?);
    let state = Arc::new(WebState {
        daemon,
        name: name.to_string(),
        password_hash,
        sessions: Mutex::default(),
        failures: Mutex::default(),
    });
    let app = router(state);
    let task = tokio::spawn(async move {
        loop {
            let Ok((tcp, peer)) = listener.accept().await else { continue };
            let (acceptor, app) = (acceptor.clone(), app.clone());
            tokio::spawn(async move {
                let Ok(Ok(tls)) = tokio::time::timeout(Duration::from_secs(10), acceptor.accept(tcp)).await else {
                    return;
                };
                let service = TowerToHyperService::new(app.layer(Extension(Peer(peer))));
                let _ = hyper::server::conn::http1::Builder::new().serve_connection(TokioIo::new(tls), service).await;
            });
        }
    });
    Ok(WebServer { task: Some(task), addr, fingerprint: fingerprint(&id.cert_der) })
}

fn router(state: Arc<WebState>) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/app.js", get(|| async { asset("text/javascript; charset=utf-8", APP_JS) }))
        .route("/web.js", get(|| async { asset("text/javascript; charset=utf-8", WEB_JS) }))
        .route("/style.css", get(|| async { asset("text/css; charset=utf-8", STYLE_CSS) }))
        .route("/logo.svg", get(|| async { asset("image/svg+xml", LOGO_SVG) }))
        .route("/api/strings", get(strings))
        .route("/api/session", get(session))
        .route("/api/login", post(login))
        .route("/api/logout", post(logout))
        .route("/api/call", post(call))
        .with_state(state)
}

fn secure(mut r: Response) -> Response {
    let h = r.headers_mut();
    h.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(CSP));
    h.insert(header::X_FRAME_OPTIONS, HeaderValue::from_static("DENY"));
    h.insert(header::X_CONTENT_TYPE_OPTIONS, HeaderValue::from_static("nosniff"));
    h.insert(header::REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    h.insert(header::STRICT_TRANSPORT_SECURITY, HeaderValue::from_static("max-age=31536000"));
    h.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    r
}

fn asset(content_type: &'static str, body: &'static str) -> Response {
    secure(([(header::CONTENT_TYPE, content_type)], body).into_response())
}

fn json<T: Serialize>(status: StatusCode, v: &T) -> Response {
    secure((status, Json(v)).into_response())
}

async fn index() -> Response {
    // The browser shim must load before the app.
    let html = INDEX_HTML.replace(
        r#"<script src="app.js"></script>"#,
        "<script src=\"web.js\"></script>\n    <script src=\"app.js\"></script>",
    );
    secure(([(header::CONTENT_TYPE, "text/html; charset=utf-8")], html).into_response())
}

#[derive(Deserialize)]
struct LangQuery {
    lang: Option<String>,
}

#[derive(Serialize)]
struct UiStrings {
    os: &'static str,
    lang: &'static str,
    languages: Vec<(&'static str, &'static str)>,
    messages: std::collections::BTreeMap<String, String>,
}

async fn strings(Query(q): Query<LangQuery>) -> Response {
    let lang = usbnexus_i18n::detect(q.lang.as_deref().filter(|l| !l.is_empty()));
    json(
        StatusCode::OK,
        &UiStrings { os: "web", lang, languages: usbnexus_i18n::languages(), messages: usbnexus_i18n::templates(lang) },
    )
}

fn cookie_token(headers: &HeaderMap) -> Option<String> {
    headers.get_all(header::COOKIE).iter().filter_map(|v| v.to_str().ok()).flat_map(|v| v.split(';')).find_map(|c| {
        let (k, v) = c.trim().split_once('=')?;
        (k == SESSION_COOKIE).then(|| v.to_string())
    })
}

impl WebState {
    fn logged_in(&self, headers: &HeaderMap) -> bool {
        let Some(token) = cookie_token(headers) else { return false };
        let mut sessions = self.sessions.lock().unwrap();
        let now = Instant::now();
        sessions.retain(|_, expiry| *expiry > now);
        sessions.contains_key(&token)
    }

    /// Seconds until `ip` may try again, if it is locked out.
    fn locked(&self, ip: IpAddr) -> Option<u64> {
        let failures = self.failures.lock().unwrap();
        let &(count, last) = failures.get(&ip)?;
        let left = LOCKOUT.checked_sub(last.elapsed())?;
        (count >= MAX_FAILURES).then(|| left.as_secs().max(1))
    }
}

#[derive(Serialize)]
struct SessionView {
    logged_in: bool,
    name: String,
}

async fn session(State(s): State<Arc<WebState>>, headers: HeaderMap) -> Response {
    json(StatusCode::OK, &SessionView { logged_in: s.logged_in(&headers), name: s.name.clone() })
}

#[derive(Deserialize)]
struct LoginBody {
    password: String,
}

#[derive(Serialize)]
struct ErrorBody {
    error: ApiError,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_after: Option<u64>,
}

fn error(status: StatusCode, code: &str, message: &str, retry_after: Option<u64>) -> Response {
    json(status, &ErrorBody { error: ApiError::new(code, message), retry_after })
}

async fn login(
    State(s): State<Arc<WebState>>,
    Extension(Peer(peer)): Extension<Peer>,
    headers: HeaderMap,
    Json(body): Json<LoginBody>,
) -> Response {
    if !headers.contains_key(CSRF_HEADER) {
        return error(StatusCode::FORBIDDEN, "invalid", "missing request header", None);
    }
    let ip = peer.ip();
    if let Some(wait) = s.locked(ip) {
        return error(StatusCode::TOO_MANY_REQUESTS, "locked", "too many failed attempts", Some(wait));
    }
    let hash = s.password_hash.clone();
    let ok = tokio::task::spawn_blocking(move || verify_password(&hash, &body.password)).await.unwrap_or(false);
    if !ok {
        let mut failures = s.failures.lock().unwrap();
        let entry = failures.entry(ip).or_insert((0, Instant::now()));
        if entry.1.elapsed() > LOCKOUT {
            entry.0 = 0;
        }
        entry.0 += 1;
        entry.1 = Instant::now();
        tracing::warn!(%ip, "failed web login");
        return error(StatusCode::UNAUTHORIZED, "wrong_password", "wrong password", None);
    }
    s.failures.lock().unwrap().remove(&ip);
    let mut raw = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut raw);
    let token = hex::encode(raw);
    s.sessions.lock().unwrap().insert(token.clone(), Instant::now() + SESSION_LIFETIME);
    let cookie = format!(
        "{SESSION_COOKIE}={token}; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age={}",
        SESSION_LIFETIME.as_secs()
    );
    let mut r = json(StatusCode::OK, &SessionView { logged_in: true, name: s.name.clone() });
    if let Ok(v) = HeaderValue::from_str(&cookie) {
        r.headers_mut().insert(header::SET_COOKIE, v);
    }
    r
}

async fn logout(State(s): State<Arc<WebState>>, headers: HeaderMap) -> Response {
    if let Some(token) = cookie_token(&headers) {
        s.sessions.lock().unwrap().remove(&token);
    }
    let mut r = secure(StatusCode::NO_CONTENT.into_response());
    r.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static("usbnexus_session=; Path=/; HttpOnly; Secure; SameSite=Strict; Max-Age=0"),
    );
    r
}

async fn call(State(s): State<Arc<WebState>>, headers: HeaderMap, body: axum::body::Bytes) -> Response {
    if !headers.contains_key(CSRF_HEADER) {
        return error(StatusCode::FORBIDDEN, "invalid", "missing request header", None);
    }
    if !s.logged_in(&headers) {
        return error(StatusCode::UNAUTHORIZED, "not_logged_in", "sign in first", None);
    }
    let req: Request = match serde_json::from_slice(&body) {
        Ok(r) => r,
        Err(e) => return error(StatusCode::BAD_REQUEST, "invalid", &e.to_string(), None),
    };
    if matches!(req, Request::WebConfigure { .. }) {
        return error(StatusCode::FORBIDDEN, "forbidden", "the web interface is configured locally", None);
    }
    let resp: ApiResponse = s.daemon.handle(req).await;
    let status = match &resp {
        ApiResponse::Ok { .. } => StatusCode::OK,
        ApiResponse::Error { .. } => StatusCode::OK, // application errors travel in the body
    };
    secure((status, Json(resp)).into_response())
}

/// Addresses the web interface can be reached at.
pub fn urls(settings: &WebSettings, port: u16, hostname: &str) -> Vec<String> {
    let mut out = vec![format!("https://localhost:{port}")];
    if settings.lan {
        let label: String =
            hostname.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' { c } else { '-' }).collect();
        if !label.is_empty() {
            out.push(format!("https://{label}.local:{port}"));
        }
        if let Ok(ifaces) = if_addrs::get_if_addrs() {
            for i in ifaces {
                if let IpAddr::V4(ip) = i.ip() {
                    if !ip.is_loopback() && !ip.is_link_local() {
                        out.push(format!("https://{ip}:{port}"));
                    }
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passwords() {
        assert!(hash_password("short").is_err());
        let h = hash_password("correct horse").unwrap();
        assert!(verify_password(&h, "correct horse"));
        assert!(!verify_password(&h, "wrong horse"));
        assert!(!verify_password("not a hash", "x"));
    }

    #[test]
    fn cookies() {
        let mut h = HeaderMap::new();
        h.insert(header::COOKIE, HeaderValue::from_static("a=1; usbnexus_session=abc; b=2"));
        assert_eq!(cookie_token(&h).as_deref(), Some("abc"));
    }

    #[test]
    fn listen_address() {
        let mut s = WebSettings::default();
        assert!(s.listen_addr().ip().is_loopback());
        s.lan = true;
        assert!(s.listen_addr().ip().is_unspecified());
        assert_eq!(s.listen_addr().port(), DEFAULT_PORT);
    }
}
