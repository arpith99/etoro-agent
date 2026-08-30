//! A one-shot local HTTP server, shared by the contract suites.
//!
//! Each integration test binary is its own crate, so this is included with
//! `mod common;` rather than imported. Every helper is `pub` for that reason;
//! `dead_code` is allowed because a given suite uses only part of it.

#![allow(dead_code)]

use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct MockResponse {
    pub base_url: String,
    pub request: Receiver<String>,
    pub server: JoinHandle<()>,
}

pub fn serve_once(status: &str, body: &'static str) -> MockResponse {
    serve_once_with_headers(status, &[], body)
}

pub fn serve_once_with_headers(
    status: &str,
    extra_headers: &[(&str, &str)],
    body: &'static str,
) -> MockResponse {
    let extra_headers: String = extra_headers
        .iter()
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .collect();
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (request_sender, request) = mpsc::channel();
    let status = status.to_owned();
    let extra_headers = extra_headers.to_owned();

    let server = thread::spawn(move || {
        // `TcpListener::accept` has no timeout, so a client that never connects
        // would park this thread forever and turn a failing test into a hang.
        // Poll in non-blocking mode against a deadline instead.
        listener.set_nonblocking(true).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut stream = loop {
            match listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < deadline, "no client connected within 5s");
                    thread::sleep(Duration::from_millis(5));
                }
                Err(error) => panic!("accept failed: {error}"),
            }
        };
        stream.set_nonblocking(false).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        let mut bytes = Vec::new();
        let mut buffer = [0_u8; 1024];
        while !bytes.windows(4).any(|window| window == b"\r\n\r\n") {
            let read = stream.read(&mut buffer).unwrap();
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..read]);
        }
        // Headers end at the blank line, but a POST body follows it. Read
        // exactly `content-length` more bytes so write contract tests can
        // assert on what was actually sent, not merely that something was.
        let head = String::from_utf8_lossy(&bytes).to_string();
        let body_start = head.find("\r\n\r\n").map_or(bytes.len(), |at| at + 4);
        let content_length: usize = head
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.trim()
                    .eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse().ok())?
            })
            .unwrap_or(0);
        while bytes.len() < body_start + content_length {
            let read = stream.read(&mut buffer).unwrap();
            if read == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..read]);
        }

        request_sender
            .send(String::from_utf8(bytes).unwrap())
            .unwrap();

        let response = format!(
            "HTTP/1.1 {status}\r\ncontent-type: application/json\r\n{extra_headers}content-length: {}\r\nconnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(response.as_bytes()).unwrap();
    });

    MockResponse {
        base_url: format!("http://{address}"),
        request,
        server,
    }
}
