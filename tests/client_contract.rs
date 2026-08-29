use etoro_agent::client::EtoroClient;
use etoro_agent::error::ApiErrorKind;
use reqwest::StatusCode;
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

const WATCHLISTS_FIXTURE: &str = include_str!("fixtures/watchlists.json");
const PORTFOLIO_FIXTURE: &str = include_str!("fixtures/portfolio.json");
const ME_FIXTURE: &str = include_str!("fixtures/me.json");

struct MockResponse {
    base_url: String,
    request: Receiver<String>,
    server: JoinHandle<()>,
}

fn serve_once(status: &str, body: &'static str) -> MockResponse {
    serve_once_with_headers(status, &[], body)
}

fn serve_once_with_headers(
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

fn assert_auth_and_request_id(request: &str) {
    let request = request.to_ascii_lowercase();
    assert!(request.contains("x-api-key: fixture-api-key\r\n"));
    assert!(request.contains("x-user-key: fixture-user-key\r\n"));
    assert!(request.contains("x-request-id: "));
}

#[tokio::test]
async fn watchlists_contract_uses_expected_path_and_headers() {
    let mock = serve_once("200 OK", WATCHLISTS_FIXTURE);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let response = client.watchlists().await.unwrap();
    assert_eq!(response.watchlists.len(), 1);
    assert_eq!(response.watchlists[0].items[0].item_id, 1001);

    let request = mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(request.starts_with("GET /api/v1/watchlists HTTP/1.1\r\n"));
    assert_auth_and_request_id(&request);
    mock.server.join().unwrap();
}

#[tokio::test]
async fn portfolio_contract_preserves_decimal_precision() {
    let mock = serve_once("200 OK", PORTFOLIO_FIXTURE);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let response = client.portfolio().await.unwrap();
    let portfolio = response.client_portfolio.unwrap();
    assert_eq!(
        portfolio.credit.unwrap().to_string(),
        "0.1234567890123456789012345678"
    );
    assert_eq!(
        portfolio.positions[0].amount.unwrap().to_string(),
        "100.0000000000000000000001"
    );

    let request = mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(request.starts_with("GET /api/v1/trading/info/portfolio HTTP/1.1\r\n"));
    assert_auth_and_request_id(&request);
    mock.server.join().unwrap();
}

#[tokio::test]
async fn me_contract_uses_expected_path_and_headers() {
    let mock = serve_once("200 OK", ME_FIXTURE);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let response = client.me().await.unwrap();

    // Upstream marks these required, so they decode as plain values rather than
    // Option. No `.unwrap()` here is the point: presence is enforced by serde
    // at the type level, which is why me() carries no hand-written checks.
    assert_eq!(response.gcid, 111111);
    assert_eq!(response.demo_cid, 222222);
    assert_eq!(response.real_cid, 333333);
    assert_eq!(response.username, "fixture-user");
    assert_eq!(response.scopes.len(), 2);

    // The API sends an empty string, not null, for an absent middle name --
    // observed on a live response. `nullable: true` in the spec makes this an
    // Option, so the absent case arrives as Some("") and never as None. Callers
    // matching on Some(..) must still handle the empty string.
    assert_eq!(response.middle_name.as_deref(), Some(""));

    let request = mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(request.starts_with("GET /api/v1/me HTTP/1.1\r\n"));
    assert_auth_and_request_id(&request);
    mock.server.join().unwrap();
}

#[tokio::test]
async fn me_missing_required_field_is_rejected_with_the_field_name() {
    // me() has no hand-written validation: every field it relies on is
    // non-Option, so serde is the only thing standing between a malformed
    // response and the caller. This pins that the resulting error is actually
    // actionable -- it must name the missing field, not just say "decode
    // failed" -- because nothing else in the method would catch it.
    let mock = serve_once(
        "200 OK",
        r#"{"realCid":333333,"demoCid":222222,"username":"fixture-user","playerLevel":1,
            "gender":0,"language":1,"dateOfBirth":"1970-01-01","scopes":[]}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = format!("{:#}", client.me().await.unwrap_err());
    assert!(error.contains("could not decode response body"), "{error}");
    assert!(error.contains("missing field `gcid`"), "{error}");
    assert!(error.contains("request ID"), "{error}");

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn http_errors_include_status_body_and_request_id() {
    let mock = serve_once("500 Internal Server Error", r#"{"message":"boom"}"#);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err();

    // A server error is worth retrying; the status is typed so callers branch on
    // `is_server_error()` rather than needing a variant per code.
    assert!(matches!(
        &error.kind,
        ApiErrorKind::Http { status, body }
            if *status == StatusCode::INTERNAL_SERVER_ERROR && body.contains("boom")
    ));
    assert!(error.is_retryable());
    assert_eq!(error.status(), Some(StatusCode::INTERNAL_SERVER_ERROR));
    assert!(error.to_string().contains("request ID"));

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn rate_limiting_is_typed_and_carries_retry_after() {
    // 429 is split out of Http because it is the only status carrying data the
    // caller needs. Retry-After must be read from the headers before the body is
    // consumed, so this also pins that ordering in get_json.
    let mock = serve_once_with_headers(
        "429 Too Many Requests",
        &[("retry-after", "30")],
        r#"{"message":"rate limited"}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err();

    assert!(matches!(error.kind, ApiErrorKind::RateLimited { .. }));
    assert_eq!(error.retry_after(), Some(Duration::from_secs(30)));
    assert!(error.is_retryable());
    assert_eq!(error.status(), Some(StatusCode::TOO_MANY_REQUESTS));

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn forbidden_is_reported_as_a_scope_problem_and_is_not_retryable() {
    // eToro returns 403, not 401, when a key lacks the scope for an endpoint.
    // Retrying that would only burn rate-limit budget.
    let mock = serve_once("403 Forbidden", r#"{"message":"insufficient scope"}"#);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err();

    assert!(matches!(&error.kind, ApiErrorKind::Forbidden { body } if body.contains("scope")));
    assert!(!error.is_retryable());
    assert!(error.to_string().contains("may lack the scope"));

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn api_level_failures_are_rejected_with_reason() {
    let mock = serve_once(
        "200 OK",
        r#"{"isSucceeded":false,"status":200,"watchlists":[],
            "exception":{"reason":"LimitExceeded","message":"Watchlist limit exceeded","invalidItems":["abc"]}}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err().to_string();
    assert!(error.contains("API-level failure"), "{error}");
    assert!(error.contains("LimitExceeded"), "{error}");
    assert!(error.contains("Watchlist limit exceeded"), "{error}");
    assert!(error.contains("abc"), "{error}");
    assert!(error.contains("request ID"), "{error}");

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn api_level_failures_without_exception_still_report() {
    let mock = serve_once(
        "200 OK",
        r#"{"isSucceeded":false,"status":200,"watchlists":[]}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err().to_string();
    assert!(error.contains("API-level failure"), "{error}");
    assert!(error.contains("no exception details"), "{error}");

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn schema_mismatch_is_reported_as_decode_failure_not_invalid_json() {
    // Valid JSON, but `watchlists` must be an array. The message must point at
    // decoding (with serde's reason and the request ID), not claim bad JSON.
    let mock = serve_once(
        "200 OK",
        r#"{"isSucceeded":true,"status":200,"watchlists":"nope"}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = format!("{:#}", client.watchlists().await.unwrap_err());
    assert!(error.contains("could not decode response body"), "{error}");
    assert!(error.contains("expected a sequence"), "{error}");
    assert!(error.contains("request ID"), "{error}");
    assert!(!error.contains("invalid JSON"), "{error}");

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[test]
fn plaintext_http_is_only_allowed_for_loopback_hosts() {
    for allowed in [
        "http://127.0.0.1:8080",
        "http://localhost:8080/prefix",
        "http://[::1]:8080",
        "https://public-api.etoro.com",
        "https://sandbox.example.com/",
    ] {
        assert!(
            EtoroClient::with_base_url("k", "u", allowed).is_ok(),
            "{allowed} should be accepted"
        );
    }
    for rejected in [
        "http://public-api.etoro.com",
        "http://10.0.0.5",
        "http://example.com",
        "ftp://127.0.0.1",
    ] {
        let error = EtoroClient::with_base_url("k", "u", rejected)
            .err()
            .unwrap_or_else(|| panic!("{rejected} should be rejected"))
            .to_string();
        assert!(error.contains("non-HTTPS"), "{error}");
    }
}
