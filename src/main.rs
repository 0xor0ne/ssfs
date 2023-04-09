//! HTTPs static file server with embedded self-signed certificate
//!
//! # Usage
//!
//! `ssfs [--port <listening_port>] [--ip <bind_address>]`
//!
//! Defaults:
//!
//! - listening_port: 8443
//! - bind_address: 0.0.0.0
//!
//! Files are served from the `ssfs` current working directory
//!
//! # Examples
//!
//! ```bash
//! ssfs --port 9000 --ip 0.0.0.0
//! ```
//!
//! ## Example output
//!
//! ```
//! Starting server at: https://0.0.0.0:9000
//! [2023-04-09T15:35:25Z INFO  actix_server::builder] starting 10 workers
//! [2023-04-09T15:35:25Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
//! [2023-04-09T15:35:33Z INFO  ssfs] 127.0.0.1 curl/7.79.1 GET /Cargo.lock HTTP/2.0 /Cargo.lock
//! ```
//!
use actix_web::{middleware::Logger, App, HttpServer};
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, read_one};

#[derive(argh::FromArgs)]
#[argh(description = "Secure Static File Server")]
/// ssfs command line options
struct Args {
    /// IP address to bind to
    #[argh(option, default = "String::from(\"0.0.0.0\")")]
    ip: String,

    /// port to bind to
    #[argh(option, default = "8443")]
    port: u16,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Args = argh::from_env();

    // Enable logger (INFO)
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Get key and cert embedded in executable
    let cert = get_cert();
    let key = get_key();

    // Setup rustls configuration
    let config = ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(cert, key)
        .expect("bad certificate/key");

    let addr = format!("{}:{}", args.ip, args.port);

    println!("Starting server at: https://{}", addr);

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%{r}a %{User-Agent}i %r").log_target("ssfs"))
            .service(
                actix_files::Files::new("/", ".")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind_rustls(addr, config)?
    .run()
    .await
    .unwrap();

    Ok(())
}

/// Return the certificate embedded in the executable
fn get_cert() -> Vec<Certificate> {
    let cert_bytes = include_bytes!("../assets/cert.pem");
    let mut cert_reader = std::io::Cursor::new(cert_bytes);

    certs(&mut cert_reader)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect()
}

/// Return the server key embedded in the executable
fn get_key() -> PrivateKey {
    let key_bytes = include_bytes!("../assets/key.pem");
    let mut key_reader = std::io::Cursor::new(key_bytes);

    match read_one(&mut key_reader).expect("cannot parse private key .pem file") {
        Some(rustls_pemfile::Item::RSAKey(key)) => PrivateKey(key),
        Some(rustls_pemfile::Item::PKCS8Key(key)) => PrivateKey(key),
        Some(rustls_pemfile::Item::ECKey(key)) => PrivateKey(key),
        _ => panic!("found non-RSA key in private key file"),
    }
}
