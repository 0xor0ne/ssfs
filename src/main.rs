use actix_files::Files;
use actix_web::{App, HttpServer};
use argh::FromArgs;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, read_one};

#[derive(FromArgs)]
/// Simple HTTPS server using rustls and actix-web
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

    let cert = load_cert();
    let key = load_key();

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
        App::new().service(
            Files::new("/", ".")
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

fn load_cert() -> Vec<Certificate> {
    let cert_bytes = include_bytes!("../assets/cert.pem");
    let mut cert_reader = std::io::Cursor::new(cert_bytes);

    certs(&mut cert_reader)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect()
}

fn load_key() -> PrivateKey {
    let key_bytes = include_bytes!("../assets/key.pem");
    println!("{:?}", key_bytes);
    let mut key_reader = std::io::Cursor::new(key_bytes);

    match read_one(&mut key_reader).expect("cannot parse private key .pem file") {
        Some(rustls_pemfile::Item::RSAKey(key)) => PrivateKey(key),
        Some(rustls_pemfile::Item::PKCS8Key(key)) => PrivateKey(key),
        Some(rustls_pemfile::Item::ECKey(key)) => PrivateKey(key),
        _ => panic!("found non-RSA key in private key file"),
    }
}
