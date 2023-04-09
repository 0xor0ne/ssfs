# Secure Static File Server

Static Files HTTPs server with self signed embedded certificate

## Installation

Install using cargo:

```bash
cargo install ssfs
```

Or build from source:

```bash
git clone https://github.com/0xor0ne/ssfs
cd ssfs
cargo build --release
# executable in target/release/ssfs
```

## Usage

General usage:

```bash
ssfs [--port <listening_port>] [--ip <binding_ip_address>]
```

`--port` and `--ip` are optional and their default values are:

- `--port`: 8443
- `--ip`: `0.0.0.0`

`ssfs` will serve files present in its current working directory and
sub-directories.

### Examples

The following examples assume `ssfs` is present in current $PATH. If this is not
the case, run `ssfs` by specifying the full path to the executable or copy
`ssfs` in the directory where the files to be served are located and run it with
`./ssfs`.

Run `ssfs` on port 9000:

```bash
ssfs --port 9000
```

You can use `curl` with the `--insecure` option to connect to the server:

```bash
curl --insecure https://<server_ip>:9000/
```

you can download a specific file by using the path to the file (e.g.
`path/to/file.txt`):

```bash
curl --insecure https://<server_ip>:9000/path/to/file.txt
```

This is an example of log printed by the server:

```bash
Starting server at: https://0.0.0.0:9000
[2023-04-09T15:59:47Z INFO  actix_server::builder] starting 10 workers
[2023-04-09T15:59:47Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime
[2023-04-09T15:59:54Z INFO  ssfs] 127.0.0.1 curl/7.79.1 GET /path/to/file.txt HTTP/2.0 /path/to/file.txt
```

You can also connect to the server using a standard web browser and set the
browser to trust the self signed certificate.

## Certificate and Key

`ssfs` comes with pre-generate server certificate and key. They are located in:

- `assets/cert.pem`
- `assets/key.pem`

These two files are embedded into the `ssfs` executable during build.

If you want to use different certificate and key the the ones provided, you can
use run the following scripts for generating a new pair of certificate and key:

```bash
./scripts/generate_cert_and_key.sh
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
