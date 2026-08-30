//! Contract tests for the Tiingo bar source, against a local one-shot server.
//!
//! No live calls and no credentials, per `docs/architecture.md`.

use chrono::NaiveDate;
use etoro_agent::data::{BarSource, DataError, DateRange, PriceBasis, tiingo::Tiingo};
use std::time::Duration;

mod common;
use common::serve_once;

const DAILY_FIXTURE: &str = include_str!("fixtures/tiingo_daily.json");

fn date(text: &str) -> NaiveDate {
    text.parse().unwrap()
}

fn range() -> DateRange {
    DateRange::new(date("2026-08-01"), date("2026-08-31")).unwrap()
}

#[tokio::test]
async fn daily_bars_request_carries_the_token_in_a_header_not_the_url() {
    let mock = serve_once("200 OK", DAILY_FIXTURE);
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    tiingo.daily_bars("AAPL", range()).await.unwrap();

    let request = mock.request.recv_timeout(Duration::from_secs(1)).unwrap();
    assert!(
        request.starts_with(
            "GET /tiingo/daily/AAPL/prices?startDate=2026-08-01&endDate=2026-08-31 HTTP/1.1\r\n"
        ),
        "unexpected request line: {request}"
    );
    assert!(
        request
            .to_ascii_lowercase()
            .contains("authorization: token fixture-token\r\n")
    );
    // Errors in this crate embed the request URL, so a token in the query
    // string would leak into logs.
    let request_line = request.lines().next().unwrap();
    assert!(!request_line.contains("fixture-token"));
    mock.server.join().unwrap();
}

#[tokio::test]
async fn rows_are_returned_ascending_regardless_of_the_order_they_arrive_in() {
    // The fixture is deliberately out of order. `BarSource` promises ascending
    // bars, so the implementation must guarantee it rather than inherit it
    // from a vendor's current habit.
    let mock = serve_once("200 OK", DAILY_FIXTURE);
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    let bars = tiingo.daily_bars("AAPL", range()).await.unwrap();

    let dates: Vec<NaiveDate> = bars.iter().map(|bar| bar.date).collect();
    assert_eq!(
        dates,
        vec![date("2026-08-26"), date("2026-08-27"), date("2026-08-28")]
    );
    mock.server.join().unwrap();
}

#[tokio::test]
async fn both_price_series_and_the_corporate_actions_survive_decoding() {
    let mock = serve_once("200 OK", DAILY_FIXTURE);
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    let bars = tiingo.daily_bars("AAPL", range()).await.unwrap();

    // Storing both is the point: an adjusted series alone cannot be turned
    // back into an unadjusted one.
    let first = &bars[0];
    assert_eq!(first.close.to_string(), "100.0000000000000000000001");
    // "90.0", not "90": Numeric keeps the scale the wire used rather than
    // normalising it away, which matters for money.
    assert_eq!(
        first.total_return_close.as_ref().unwrap().to_string(),
        "90.0"
    );

    // Scientific notation, and a row where the vendor sent nulls.
    let second = &bars[1];
    assert_eq!(second.close.to_string(), "0.00000506");
    assert!(second.volume.is_none());
    assert!(second.total_return_close.is_none());
    assert_eq!(second.dividend_cash.as_ref().unwrap().to_string(), "0.25");

    let third = &bars[2];
    assert_eq!(third.split_factor.as_ref().unwrap().to_string(), "3.0");
    // Volume is split-adjusted separately from price: a 3:1 split triples the
    // share count for the same money, and a dividend would move price without
    // moving volume at all.
    assert_eq!(
        third.split_adjusted_volume.as_ref().unwrap().to_string(),
        "9000000"
    );
    assert!(second.split_adjusted_volume.is_none());
    mock.server.join().unwrap();
}

#[tokio::test]
async fn tiingo_reports_as_traded_prices_unlike_etoro() {
    let tiingo = Tiingo::new("fixture-token").unwrap();
    // Not a formality: eToro's candles are SplitAdjusted, so a caller mixing
    // the two sources without consulting `basis()` compares different things.
    assert_eq!(tiingo.basis(), PriceBasis::AsTraded);
    assert_eq!(tiingo.name(), "tiingo");
}

#[tokio::test]
async fn an_unknown_ticker_is_distinct_from_an_empty_range() {
    let mock = serve_once("404 Not Found", r#"{"detail":"Error: Ticker not found."}"#);
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    match tiingo.daily_bars("NOSUCH", range()).await.unwrap_err() {
        DataError::UnknownSymbol { symbol, .. } => assert_eq!(symbol, "NOSUCH"),
        other => panic!("expected UnknownSymbol, got {other:?}"),
    }
    mock.server.join().unwrap();
}

#[tokio::test]
async fn a_rejected_token_is_distinct_from_a_transport_failure() {
    let mock = serve_once("401 Unauthorized", r#"{"detail":"Invalid token."}"#);
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    match tiingo.daily_bars("AAPL", range()).await.unwrap_err() {
        // The fix is configuration; retrying cannot help.
        DataError::Unauthorized { detail, .. } => assert!(detail.contains("Invalid token")),
        other => panic!("expected Unauthorized, got {other:?}"),
    }
    mock.server.join().unwrap();
}

#[tokio::test]
async fn an_empty_range_is_not_an_error() {
    let mock = serve_once("200 OK", "[]");
    let tiingo = Tiingo::with_base_url("fixture-token", &mock.base_url).unwrap();

    // A quiet range, an instrument listed later, a holiday week: all normal.
    assert!(tiingo.daily_bars("AAPL", range()).await.unwrap().is_empty());
    mock.server.join().unwrap();
}

#[tokio::test]
async fn a_ticker_that_could_escape_the_url_path_is_refused_before_sending() {
    // Nothing listens on port 1, so reaching the network at all would fail.
    let tiingo = Tiingo::with_base_url("fixture-token", "http://127.0.0.1:1").unwrap();

    match tiingo.daily_bars("../../admin", range()).await.unwrap_err() {
        DataError::InvalidRequest { detail, .. } => assert!(detail.contains("admin")),
        other => panic!("expected InvalidRequest, got {other:?}"),
    }
}
