// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! Web interface over real HTTPS: sign-in, sessions, CSRF header, rate
//! limiting and the local-only restriction on reconfiguration.

use std::sync::Arc;

use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use rustls::{DigitallySignedStruct, SignatureScheme};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use usbnexus_core::api::{Request, Response, WebStatusView};
use usbnexus_core::backend::demo::{DemoHost, DemoImport};
use usbnexus_core::daemon::{Daemon, DaemonOptions};

/// Accepts the self-signed certificate, like a user clicking through the warning.
#[derive(Debug)]
struct AcceptAny;

impl ServerCertVerifier for AcceptAny {
    fn verify_server_cert(
        &self,
        _: &CertificateDer<'_>,
        _: &[CertificateDer<'_>],
        _: &ServerName<'_>,
        _: &[u8],
        _: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }
    fn verify_tls12_signature(
        &self,
        _: &[u8],
        _: &CertificateDer<'_>,
        _: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn verify_tls13_signature(
        &self,
        _: &[u8],
        _: &CertificateDer<'_>,
        _: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }
    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        rustls::crypto::ring::default_provider().signature_verification_algorithms.supported_schemes()
    }
}

struct Reply {
    status: u16,
    headers: String,
    body: String,
}

/// One HTTP/1.1 request over TLS (connection closed afterwards).
async fn request(port: u16, method: &str, path: &str, headers: &[(&str, &str)], body: &str) -> Reply {
    let cfg = rustls::ClientConfig::builder_with_provider(Arc::new(rustls::crypto::ring::default_provider()))
        .with_safe_default_protocol_versions()
        .unwrap()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(AcceptAny))
        .with_no_client_auth();
    let tcp = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.unwrap();
    let mut tls = tokio_rustls::TlsConnector::from(Arc::new(cfg))
        .connect(ServerName::try_from("localhost").unwrap(), tcp)
        .await
        .unwrap();
    let mut req = format!(
        "{method} {path} HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\nContent-Length: {}\r\n",
        body.len()
    );
    for (k, v) in headers {
        req.push_str(&format!("{k}: {v}\r\n"));
    }
    req.push_str("\r\n");
    req.push_str(body);
    tls.write_all(req.as_bytes()).await.unwrap();
    let mut raw = Vec::new();
    let _ = tls.read_to_end(&mut raw).await;
    let text = String::from_utf8_lossy(&raw).into_owned();
    let (head, body) = text.split_once("\r\n\r\n").unwrap_or((&text, ""));
    let status = head.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    Reply { status, headers: head.to_lowercase(), body: body.to_string() }
}

const JSON: (&str, &str) = ("Content-Type", "application/json");
const CSRF: (&str, &str) = ("X-USBNexus", "1");

async fn start() -> (Daemon, u16, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let d = Daemon::start(DaemonOptions {
        state_dir: dir.path().to_path_buf(),
        name: "web-test".into(),
        listen: "127.0.0.1:0".into(),
        mdns: false,
        host: Arc::new(DemoHost),
        import: Arc::new(DemoImport::default()),
        events: None,
    })
    .await
    .unwrap();
    let status = match d
        .handle(Request::WebConfigure {
            enabled: Some(true),
            lan: Some(false),
            port: Some(0),
            password: Some("correct horse".into()),
        })
        .await
    {
        Response::Ok { data } => serde_json::from_value::<WebStatusView>(data).unwrap(),
        Response::Error { error } => panic!("{error:?}"),
    };
    assert!(status.error.is_none(), "{:?}", status.error);
    assert!(status.fingerprint.is_some());
    (d, status.port, dir)
}

fn session_cookie(r: &Reply) -> String {
    let line = r.headers.lines().find(|l| l.starts_with("set-cookie:")).expect("cookie set");
    assert!(line.contains("httponly") && line.contains("secure") && line.contains("samesite=strict"));
    line["set-cookie:".len()..].trim().split(';').next().unwrap().to_string()
}

#[tokio::test]
async fn sign_in_and_use_the_api() {
    let (_d, port, _dir) = start().await;

    let page = request(port, "GET", "/", &[], "").await;
    assert_eq!(page.status, 200);
    assert!(page.body.contains("web.js"));
    assert!(page.headers.contains("content-security-policy: default-src 'self'"));
    assert!(page.headers.contains("x-frame-options: deny"));

    let call = r#"{"cmd":"local_devices"}"#;
    assert_eq!(request(port, "POST", "/api/call", &[CSRF], call).await.status, 401, "needs a session");

    let good = r#"{"password":"correct horse"}"#;
    assert_eq!(request(port, "POST", "/api/login", &[JSON], good).await.status, 403, "needs the header");
    let bad = r#"{"password":"nope"}"#;
    assert_eq!(request(port, "POST", "/api/login", &[JSON, CSRF], bad).await.status, 401);

    let login = request(port, "POST", "/api/login", &[JSON, CSRF], good).await;
    assert_eq!(login.status, 200);
    let cookie = session_cookie(&login);

    let r = request(port, "POST", "/api/call", &[CSRF, ("Cookie", &cookie)], call).await;
    assert_eq!(r.status, 200);
    let v: serde_json::Value = serde_json::from_str(&r.body).unwrap();
    assert_eq!(v["status"], "ok");
    assert_eq!(v["data"].as_array().unwrap().len(), 4);

    assert_eq!(
        request(port, "POST", "/api/call", &[("Cookie", &cookie)], call).await.status,
        403,
        "API calls need the header even with a session"
    );
    // Reconfiguring from the web interface restarts the server (here on a
    // new random port); the session survives the restart.
    let reconfigure = r#"{"cmd":"web_configure","lan":true}"#;
    let r = request(port, "POST", "/api/call", &[CSRF, ("Cookie", &cookie)], reconfigure).await;
    assert_eq!(r.status, 200, "{}", r.body);
    let v: serde_json::Value = serde_json::from_str(&r.body).unwrap();
    assert_eq!(v["status"], "ok", "{}", r.body);
    let status: WebStatusView = serde_json::from_value(v["data"].clone()).unwrap();
    assert!(status.lan);
    let port = status.port;
    assert_eq!(request(port, "POST", "/api/call", &[CSRF, ("Cookie", &cookie)], call).await.status, 200);

    assert_eq!(request(port, "POST", "/api/logout", &[("Cookie", &cookie)], "").await.status, 204);
    assert_eq!(request(port, "POST", "/api/call", &[CSRF, ("Cookie", &cookie)], call).await.status, 401);
}

#[tokio::test]
async fn repeated_failures_lock_out() {
    let (_d, port, _dir) = start().await;
    let bad = r#"{"password":"nope"}"#;
    for _ in 0..5 {
        assert_eq!(request(port, "POST", "/api/login", &[JSON, CSRF], bad).await.status, 401);
    }
    let good = r#"{"password":"correct horse"}"#;
    let r = request(port, "POST", "/api/login", &[JSON, CSRF], good).await;
    assert_eq!(r.status, 429, "even the right password waits");
    assert!(r.body.contains("retry_after"));
}

#[tokio::test]
async fn disabling_stops_the_server_and_passwords_are_required() {
    let (d, port, _dir) = start().await;
    let r = d.handle(Request::WebConfigure { enabled: Some(false), lan: None, port: None, password: None }).await;
    assert!(matches!(r, Response::Ok { .. }));
    assert!(tokio::net::TcpStream::connect(("127.0.0.1", port)).await.is_err(), "listener closed");

    let dir = tempfile::tempdir().unwrap();
    let fresh = Daemon::start(DaemonOptions {
        state_dir: dir.path().to_path_buf(),
        name: "fresh".into(),
        listen: "127.0.0.1:0".into(),
        mdns: false,
        host: Arc::new(DemoHost),
        import: Arc::new(DemoImport::default()),
        events: None,
    })
    .await
    .unwrap();
    match fresh.handle(Request::WebConfigure { enabled: Some(true), lan: None, port: None, password: None }).await {
        Response::Error { error } => assert_eq!(error.code, "password_required"),
        other => panic!("{other:?}"),
    }
}

#[tokio::test]
async fn plain_http_is_redirected_to_https() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let (_d, port, _dir) = start().await;
    let plain = |host: String| async move {
        let mut s = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.unwrap();
        s.write_all(format!("GET /x HTTP/1.1\r\nHost: {host}\r\n\r\n").as_bytes()).await.unwrap();
        let mut reply = String::new();
        s.read_to_string(&mut reply).await.unwrap();
        reply
    };
    let reply = plain(format!("localhost:{port}")).await;
    assert!(reply.starts_with("HTTP/1.1 301"), "{reply}");
    assert!(reply.contains(&format!("Location: https://localhost:{port}/\r\n")), "{reply}");
    // A header line cannot be smuggled in through the Host header.
    let reply = plain("evil\r\nSet-Cookie: x=1".into()).await;
    assert!(reply.contains("Location: https://evil/\r\n"), "{reply}");
    assert!(!reply.contains("Set-Cookie"), "{reply}");
    let reply = plain("bad host<script>".into()).await;
    assert!(reply.contains(&format!("Location: https://localhost:{port}/\r\n")), "{reply}");
    // HTTPS still works on the same port.
    assert_eq!(request(port, "GET", "/", &[], "").await.status, 200);
}
