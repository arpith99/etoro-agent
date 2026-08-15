use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};

use etoro_agent::client::EtoroClient;

const WATCHLISTS_FIXTURE: &str = include_str!("fixtures/watchlists.json");
const PORTFOLIO_FIXTURE: &str = include_str!("fixtures/portfolio.json");

struct MockResponse {
    base_url: String,
    request: Receiver<String>,
    server: JoinHandle<()>,
}

fn serve_once(status: &str, body: &'static str) -> MockResponse {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let (request_sender, request) = mpsc::channel();
    let status = status.to_owned();

    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
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
            "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{body}",
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
async fn http_errors_include_status_body_and_request_id() {
    let mock = serve_once("429 Too Many Requests", r#"{"message":"rate limited"}"#);
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err().to_string();
    assert!(error.contains("429 Too Many Requests"));
    assert!(error.contains("rate limited"));
    assert!(error.contains("request ID"));

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}

#[tokio::test]
async fn api_level_failures_are_rejected() {
    let mock = serve_once(
        "200 OK",
        r#"{"isSucceeded":false,"status":200,"watchlists":[]}"#,
    );
    let client =
        EtoroClient::with_base_url("fixture-api-key", "fixture-user-key", &mock.base_url).unwrap();

    let error = client.watchlists().await.unwrap_err().to_string();
    assert!(error.contains("API-level failure"));
    assert!(error.contains("request ID"));

    mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    mock.server.join().unwrap();
}
