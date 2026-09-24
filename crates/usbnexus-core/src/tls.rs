// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 USB Nexus contributors

//! TLS 1.3 configuration with mutual authentication.
//!
//! Both sides present self-signed certificates. The TLS layer only checks
//! that each peer owns the private key of the certificate it presents; the
//! decision whether that certificate is *trusted* is taken by the
//! application after the handshake, by comparing the fingerprint with the
//! trust store (or by running PIN pairing).

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::crypto::{verify_tls12_signature, verify_tls13_signature, CryptoProvider, WebPkiSupportedAlgorithms};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName, UnixTime};
use rustls::server::danger::{ClientCertVerified, ClientCertVerifier};
use rustls::{DigitallySignedStruct, DistinguishedName, SignatureScheme};

use crate::identity::Identity;

/// Label for TLS exporter keying material used to bind pairing to the session.
pub const EXPORTER_LABEL: &[u8] = b"EXPORTER-usbnexus-pairing-v1";

fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::ring::default_provider())
}

/// Accepts any certificate whose handshake signature is valid.
#[derive(Debug)]
struct PinnedLater {
    algs: WebPkiSupportedAlgorithms,
}

impl PinnedLater {
    fn new(p: &CryptoProvider) -> Self {
        PinnedLater { algs: p.signature_verification_algorithms }
    }
}

impl ServerCertVerifier for PinnedLater {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &self.algs)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &self.algs)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.algs.supported_schemes()
    }
}

impl ClientCertVerifier for PinnedLater {
    fn root_hint_subjects(&self) -> &[DistinguishedName] {
        &[]
    }

    fn verify_client_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _now: UnixTime,
    ) -> Result<ClientCertVerified, rustls::Error> {
        Ok(ClientCertVerified::assertion())
    }

    fn client_auth_mandatory(&self) -> bool {
        true
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls12_signature(message, cert, dss, &self.algs)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        verify_tls13_signature(message, cert, dss, &self.algs)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.algs.supported_schemes()
    }
}

fn cert_and_key(id: &Identity) -> (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>) {
    (
        vec![CertificateDer::from(id.cert_der.clone())],
        PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(id.key_der.clone())),
    )
}

pub fn server_config(id: &Identity) -> Result<Arc<rustls::ServerConfig>> {
    let p = provider();
    let (certs, key) = cert_and_key(id);
    let cfg = rustls::ServerConfig::builder_with_provider(p.clone())
        .with_protocol_versions(&[&rustls::version::TLS13])?
        .with_client_cert_verifier(Arc::new(PinnedLater::new(&p)))
        .with_single_cert(certs, key)
        .context("loading server certificate")?;
    Ok(Arc::new(cfg))
}

pub fn client_config(id: &Identity) -> Result<Arc<rustls::ClientConfig>> {
    let p = provider();
    let (certs, key) = cert_and_key(id);
    let cfg = rustls::ClientConfig::builder_with_provider(p.clone())
        .with_protocol_versions(&[&rustls::version::TLS13])?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(PinnedLater::new(&p)))
        .with_client_auth_cert(certs, key)
        .context("loading client certificate")?;
    Ok(Arc::new(cfg))
}

/// Server name sent in SNI. Certificates are pinned, so it is not checked.
pub fn server_name() -> ServerName<'static> {
    ServerName::try_from("usbnexus.invalid").expect("static name is valid")
}

/// Extracts the peer fingerprint from an established connection.
pub fn peer_fingerprint(conn: &rustls::CommonState) -> Result<String> {
    let certs = conn.peer_certificates().ok_or_else(|| anyhow!("peer sent no certificate"))?;
    let leaf = certs.first().ok_or_else(|| anyhow!("peer sent no certificate"))?;
    Ok(crate::identity::fingerprint(leaf))
}

/// Keying material unique to this TLS session (client side). It depends on
/// the full handshake transcript, including both certificates.
pub fn session_binding_client(conn: &rustls::ClientConnection) -> Result<[u8; 32]> {
    Ok(conn.export_keying_material([0u8; 32], EXPORTER_LABEL, None)?)
}

/// Keying material unique to this TLS session (server side).
pub fn session_binding_server(conn: &rustls::ServerConnection) -> Result<[u8; 32]> {
    Ok(conn.export_keying_material([0u8; 32], EXPORTER_LABEL, None)?)
}
