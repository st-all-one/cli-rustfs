//! End-to-end benchmarks for the real `rc` binary.
//!
//! Local scenarios exercise argument parsing, config I/O, serde, output and the
//! process/runtime startup cost. Network scenarios use a deterministic one-host
//! mock S3 endpoint so the measured path includes AWS config resolution, HTTP,
//! XML decoding and output rendering without depending on a live service.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use rc_core::config::Defaults;
use rc_core::{Alias, Config};

use crate::harness::Harness;

const OBJECT_COUNT: usize = 1_000;
const BUCKET_COUNT: usize = 32;

pub fn run(harness: &mut Harness, rc: &Path, samples: usize) {
    let temp = tempfile::tempdir().expect("create e2e temp directory");

    let startup_config = temp.path().join("startup");
    write_config(&startup_config, &[]);

    harness.measure_cmd("e2e/startup", "--version", samples, || {
        timed(rc, &["--version"], &startup_config).0
    });
    harness.measure_cmd("e2e/startup", "--help", samples, || {
        timed(rc, &["--help"], &startup_config).0
    });
    harness.measure_cmd("e2e/startup", "completions bash", samples, || {
        timed(rc, &["completions", "bash"], &startup_config).0
    });

    let set_config = temp.path().join("alias-set");
    harness.measure_cmd("e2e/alias", "set anonymous", samples, || {
        let _ = std::fs::remove_file(set_config.join("config.toml"));
        timed(
            rc,
            &[
                "alias",
                "set",
                "bench",
                "http://127.0.0.1:1",
                "--anonymous",
            ],
            &set_config,
        )
        .0
    });

    let remove_config = temp.path().join("alias-remove");
    harness.measure_cmd("e2e/alias", "remove", samples, || {
        write_config(&remove_config, &[fixture_alias("bench", "http://127.0.0.1:1")]);
        timed(rc, &["alias", "remove", "bench"], &remove_config).0
    });

    let list_config = temp.path().join("alias-list");
    let many_aliases: Vec<Alias> = (0..128)
        .map(|index| {
            fixture_alias(
                &format!("alias-{index:03}"),
                &format!("http://127.0.0.1:{}", 9000 + index),
            )
        })
        .collect();
    write_config(&list_config, &many_aliases);
    harness.measure_cmd("e2e/alias", "list 128 aliases", samples, || {
        timed(rc, &["alias", "list", "--long"], &list_config).0
    });

    let error_config = temp.path().join("local-error");
    write_config(&error_config, &[]);
    let source = temp.path().join("source.txt");
    let target = temp.path().join("target.txt");
    std::fs::write(&source, b"source").expect("write source fixture");
    std::fs::write(&target, b"target").expect("write target fixture");
    harness.measure_cmd("e2e/local", "cp local-to-local error", samples, || {
        let (duration, output) = timed(
            rc,
            &[
                "cp",
                source.to_str().expect("source path"),
                target.to_str().expect("target path"),
                "--json",
            ],
            &error_config,
        );
        assert_eq!(output.status.code(), Some(2), "unexpected cp exit code");
        duration
    });

    let server = MockS3Server::start(BUCKET_COUNT, OBJECT_COUNT);
    let s3_config = temp.path().join("s3");
    write_config(
        &s3_config,
        &[fixture_alias("bench", &server.endpoint())],
    );

    harness.measure_cmd("e2e/s3", "ls buckets json", samples, || {
        let (duration, output) = timed(rc, &["ls", "bench/", "--json"], &s3_config);
        assert!(output.status.success(), "ls buckets failed: {output:?}");
        duration
    });
    harness.measure_cmd("e2e/s3", "ls 1000 objects json", samples, || {
        let (duration, output) = timed(rc, &["ls", "bench/bucket", "--json"], &s3_config);
        assert!(output.status.success(), "ls objects failed: {output:?}");
        duration
    });
}

fn fixture_alias(name: &str, endpoint: &str) -> Alias {
    let mut alias = Alias::new(name, endpoint, "", "");
    alias.anonymous = true;
    alias
}

fn write_config(directory: &Path, aliases: &[Alias]) {
    std::fs::create_dir_all(directory).expect("create config directory");
    let config = Config {
        schema_version: 1,
        defaults: Defaults::default(),
        aliases: aliases.to_vec(),
    };
    let text = toml::to_string(&config).expect("serialize bench config");
    std::fs::write(directory.join("config.toml"), text).expect("write bench config");
}

fn timed(rc: &Path, args: &[&str], config_dir: &Path) -> (Duration, std::process::Output) {
    let mut command = Command::new(rc);
    command
        .args(args)
        .env("RC_CONFIG_DIR", config_dir)
        .env("NO_COLOR", "1")
        .env("RUST_BACKTRACE", "0")
        .env_remove("XDG_CONFIG_HOME");

    for (key, _) in std::env::vars_os() {
        if key.to_string_lossy().starts_with("RC_HOST_") {
            command.env_remove(key);
        }
    }

    let start = Instant::now();
    let output = command.output().expect("execute rc benchmark command");
    (start.elapsed(), output)
}

struct MockS3Server {
    address: SocketAddr,
    stop: Arc<AtomicBool>,
    handle: Option<JoinHandle<()>>,
}

impl MockS3Server {
    fn start(bucket_count: usize, object_count: usize) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock S3 server");
        listener
            .set_nonblocking(true)
            .expect("set mock S3 listener nonblocking");
        let address = listener.local_addr().expect("mock S3 local address");
        let stop = Arc::new(AtomicBool::new(false));
        let stop_for_thread = Arc::clone(&stop);
        let buckets = Arc::new(bucket_list_xml(bucket_count));
        let objects = Arc::new(object_list_xml(object_count));

        let handle = thread::spawn(move || {
            while !stop_for_thread.load(Ordering::Relaxed) {
                match listener.accept() {
                    Ok((stream, _peer)) => serve(stream, &buckets, &objects),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(_) => break,
                }
            }
        });

        Self {
            address,
            stop,
            handle: Some(handle),
        }
    }

    fn endpoint(&self) -> String {
        format!("http://{}", self.address)
    }
}

impl Drop for MockS3Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        let _ = TcpStream::connect(self.address);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn serve(mut stream: TcpStream, buckets: &[u8], objects: &[u8]) {
    let _ = stream.set_read_timeout(Some(Duration::from_secs(5)));
    let _ = stream.set_write_timeout(Some(Duration::from_secs(5)));

    let mut request = Vec::with_capacity(4096);
    let mut buffer = [0_u8; 4096];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => return,
            Ok(read) => {
                request.extend_from_slice(&buffer[..read]);
                if request.windows(4).any(|window| window == b"\r\n\r\n") {
                    break;
                }
            }
            Err(_) => return,
        }
    }

    let request_line = String::from_utf8_lossy(&request)
        .lines()
        .next()
        .unwrap_or_default()
        .to_string();
    let target = request_line
        .split_whitespace()
        .nth(1)
        .unwrap_or("/")
        .to_string();

    let body = if target.starts_with("/bucket") {
        objects
    } else {
        buckets
    };

    let header = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/xml\r\ncontent-length: {}\r\nconnection: close\r\nx-amz-request-id: bench\r\n\r\n",
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    let _ = stream.write_all(body);
    let _ = stream.flush();
}

fn bucket_list_xml(bucket_count: usize) -> Vec<u8> {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
         <ListAllMyBucketsResult xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">\
         <Owner><ID>bench-owner</ID><DisplayName>bench</DisplayName></Owner><Buckets>",
    );
    for index in 0..bucket_count {
        xml.push_str(&format!(
            "<Bucket><Name>bench-bucket-{index:03}</Name><CreationDate>2024-01-01T00:00:00.000Z</CreationDate></Bucket>"
        ));
    }
    xml.push_str("</Buckets></ListAllMyBucketsResult>");
    xml.into_bytes()
}

fn object_list_xml(object_count: usize) -> Vec<u8> {
    let mut xml = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\
         <ListBucketResult xmlns=\"http://s3.amazonaws.com/doc/2006-03-01/\">\
         <Name>bench-bucket</Name><Prefix></Prefix><KeyCount>{object_count}</KeyCount>\
         <MaxKeys>1000</MaxKeys><IsTruncated>false</IsTruncated>"
    );
    for index in 0..object_count {
        xml.push_str(&format!(
            "<Contents><Key>logs/2026/10/26/part-{index:04}.jsonl</Key>\
             <LastModified>2026-10-26T12:00:{:02}.000Z</LastModified>\
             <ETag>\"0123456789abcdef0123456789abcdef\"</ETag>\
             <Size>{}</Size><StorageClass>STANDARD</StorageClass></Contents>",
            index % 60,
            (index + 1) * 1024,
        ));
    }
    xml.push_str("</ListBucketResult>");
    xml.into_bytes()
}
