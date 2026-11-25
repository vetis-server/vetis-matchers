use crate::common::{CA_CERT, SERVER_CERT, SERVER_KEY};
use caramelo::{
    expect,
    matchers::{eq, truthy},
};
use deboa::{
    cert::{CertificateExt as _, ContentEncoding},
    request,
};
use deboa_tokio::cert::DeboaCertificate;
use http::{StatusCode, Version};
use serde::Deserialize;
use std::{
    net::{IpAddr, Ipv4Addr},
    path::Path,
};
use tokio::fs::read_to_string;
use vetis::{VetisServer, VetisTestResult, host::HostConfig, security::TlsConfig};
use vetis_tokio::{ServerConfig, Vetis, host::Host};

#[allow(unused)]
#[derive(Deserialize, Clone)]
pub struct VetisConfig {
    log_level: String,
    worker_threads: usize,
    max_blocking_threads: usize,
    server: ServerConfig,
}

#[tokio::test]
async fn test_from_file() -> VetisTestResult<()> {
    let config_path = Path::new("files/public/vetis-static.yaml");
    if config_path.exists() {
        let file = read_to_string(&config_path).await;
        if let Ok(file) = file {
            let config = serde_yaml_ng::from_str::<VetisConfig>(&file);
            if let Ok(config) = config {
                let config_clone = config
                    .server
                    .clone();
                expect(
                    config_clone
                        .hosts()
                        .len(),
                )
                .to_be(eq(1));

                let host = &config_clone.hosts()[0];
                expect(host.hostname()).to_be(eq("localhost"));
                expect(
                    host.hooks()
                        .is_some(),
                )
                .to_be(truthy());

                let mut server = Vetis::from_config(config.server).await?;
                server
                    .start()
                    .await?;

                let cert = tokio::fs::read(CA_CERT).await?;
                let client = deboa_tokio::Client::builder()
                    .certificate(DeboaCertificate::from_slice(&cert, ContentEncoding::DER))
                    .build();

                let request = request::get("https://localhost:8443/")?
                    .version(Version::HTTP_2)
                    .send_with(&client)
                    .await?;

                assert_eq!(request.status(), http::StatusCode::OK);

                server
                    .stop()
                    .await?;
            } else {
                eprintln!(
                    "Could not read : {}",
                    config
                        .err()
                        .unwrap()
                );
            }
        } else {
            eprintln!("Failed to start server: {}", config_path.display());
        }
    } else {
        eprintln!("Failed to start server: Config file does not exist: {}", config_path.display());
    }

    Ok(())
}

#[tokio::test]
async fn test_not_found() -> VetisTestResult<()> {
    let tls_config = TlsConfig::builder()
        .ca_file(CA_CERT)
        .cert_file(SERVER_CERT)
        .key_file(SERVER_KEY)
        .build()?;

    let host_config = HostConfig::builder()
        .hostname("localhost")
        .tls(tls_config)
        .add_proto(Version::HTTP_11)
        .add_bind_address(Ipv4Addr::UNSPECIFIED, 9000)
        .build()?;

    let mut server = Vetis::builder()
        .add_host(host_config)
        .build();

    server
        .start()
        .await?;

    let cert = tokio::fs::read(CA_CERT).await?;
    let client = deboa_tokio::Client::builder()
        .certificate(DeboaCertificate::from_slice(&cert, ContentEncoding::DER))
        .build();

    let request = request::get("https://localhost:9000/some/file/here.txt")?
        .version(Version::HTTP_11)
        .send_with(&client)
        .await?;

    expect(request.status()).to_be(eq(StatusCode::NOT_FOUND));

    server
        .stop()
        .await?;

    Ok(())
}
