//! Tiingo as a [`BarSource`].
//!
//! Chosen over the alternatives for one property: it returns `close` *and*
//! `adjClose` per bar, plus the `divCash` and `splitFactor` that explain the
//! difference. That makes it a referee -- a series any other source can be
//! validated against. A vendor returning only a price leaves you holding an
//! unlabelled number, which is the problem, not a solution to it.

use chrono::{DateTime, Utc};
use reqwest::{StatusCode, Url, header};
use serde::Deserialize;

use super::{Bar, BarSource, DataError, DateRange, PriceBasis};
use crate::client::is_loopback;
use crate::error::ClientError;
use crate::types::manual::Numeric;

const BASE_URL: &str = "https://api.tiingo.com";
const REQUEST_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const ERROR_BODY_LIMIT: usize = 512;
const SOURCE_NAME: &str = "tiingo";

/// Characters a ticker may contain.
///
/// The symbol becomes a URL path segment, so an unchecked value could climb
/// out of the path with `../`. Tiingo tickers are alphanumeric with `.` and `-`
/// separators, so anything else is a caller bug worth naming.
fn is_valid_symbol(symbol: &str) -> bool {
    !symbol.is_empty()
        && symbol
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
}

#[derive(Clone)]
pub struct Tiingo {
    http: reqwest::Client,
    base_url: Url,
}

impl Tiingo {
    pub fn new(token: &str) -> Result<Self, ClientError> {
        Self::with_base_url(token, BASE_URL)
    }

    /// Constructs a client against a custom origin, for contract tests.
    ///
    /// The token is a credential, so the same rule as
    /// [`EtoroClient::with_base_url`](crate::client::EtoroClient::with_base_url)
    /// applies: `https` only, except plain `http` on loopback.
    pub fn with_base_url(token: &str, base_url: &str) -> Result<Self, ClientError> {
        // Sent as a header rather than the `token=` query parameter Tiingo also
        // accepts. Errors in this crate carry the request URL, so a token in
        // the query string would end up in log lines and error messages.
        let mut value =
            header::HeaderValue::from_str(&format!("Token {token}")).map_err(|source| {
                ClientError::InvalidCredential {
                    header: "authorization",
                    source,
                }
            })?;
        value.set_sensitive(true);
        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, value);

        let normalized = format!("{}/", base_url.trim_end_matches('/'));
        let base_url: Url = normalized
            .parse()
            .map_err(|source| ClientError::InvalidBaseUrl {
                url: normalized.clone(),
                source,
            })?;
        if !(base_url.scheme() == "https"
            || (base_url.scheme() == "http" && is_loopback(&base_url)))
        {
            return Err(ClientError::InsecureBaseUrl {
                url: base_url.to_string(),
            });
        }

        Ok(Self {
            http: reqwest::Client::builder()
                .default_headers(headers)
                .timeout(REQUEST_TIMEOUT)
                .build()?,
            base_url,
        })
    }
}

impl BarSource for Tiingo {
    /// Tiingo's `close` is the price as it traded, so a split shows as a
    /// discontinuity. Its `adjClose` -- carried on [`Bar::total_return_close`]
    /// -- is the adjusted one. Note this differs from eToro, whose candles are
    /// already split-adjusted.
    fn basis(&self) -> PriceBasis {
        PriceBasis::AsTraded
    }

    fn name(&self) -> &'static str {
        SOURCE_NAME
    }

    async fn daily_bars(&self, symbol: &str, range: DateRange) -> Result<Vec<Bar>, DataError> {
        if !is_valid_symbol(symbol) {
            return Err(DataError::InvalidRequest {
                source_name: SOURCE_NAME,
                detail: format!("{symbol:?} is not a usable ticker"),
            });
        }

        let mut url = self
            .base_url
            .join(&format!("tiingo/daily/{symbol}/prices"))
            .map_err(|error| DataError::InvalidRequest {
                source_name: SOURCE_NAME,
                detail: format!("could not build a URL for {symbol:?}: {error}"),
            })?;
        url.query_pairs_mut()
            .append_pair("startDate", &range.start.to_string())
            .append_pair("endDate", &range.end.to_string());

        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|error| DataError::Transport {
                source_name: SOURCE_NAME,
                detail: error.to_string(),
            })?;

        let status = response.status();
        let body = response
            .bytes()
            .await
            .map_err(|error| DataError::Transport {
                source_name: SOURCE_NAME,
                detail: error.to_string(),
            })?;

        if !status.is_success() {
            let detail = body_excerpt(&body);
            return Err(match status {
                // Tiingo answers an unrecognised ticker with 404, which is a
                // different problem from an empty range and deserves saying so.
                StatusCode::NOT_FOUND => DataError::UnknownSymbol {
                    source_name: SOURCE_NAME,
                    symbol: symbol.to_owned(),
                },
                StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => DataError::Unauthorized {
                    source_name: SOURCE_NAME,
                    detail,
                },
                status => DataError::Http {
                    source_name: SOURCE_NAME,
                    status,
                    detail,
                },
            });
        }

        let rows: Vec<Row> =
            serde_json::from_slice(&body).map_err(|error| DataError::Malformed {
                source_name: SOURCE_NAME,
                detail: error.to_string(),
            })?;

        let mut bars: Vec<Bar> = rows.into_iter().map(Row::into_bar).collect();
        // The trait promises ascending and duplicate-free. Tiingo already
        // returns ascending, but a contract the caller relies on should be
        // enforced by the implementation rather than inherited from a habit.
        bars.sort_by_key(|bar| bar.date);
        bars.dedup_by_key(|bar| bar.date);
        Ok(bars)
    }
}

/// One row of Tiingo's daily price response.
#[derive(Debug, Deserialize)]
struct Row {
    date: DateTime<Utc>,
    open: Numeric,
    high: Numeric,
    low: Numeric,
    close: Numeric,
    volume: Option<Numeric>,
    #[serde(rename = "adjClose")]
    adj_close: Option<Numeric>,
    #[serde(rename = "divCash")]
    div_cash: Option<Numeric>,
    #[serde(rename = "splitFactor")]
    split_factor: Option<Numeric>,
    #[serde(rename = "adjVolume")]
    adj_volume: Option<Numeric>,
}

impl Row {
    fn into_bar(self) -> Bar {
        Bar {
            // Tiingo timestamps every daily row at midnight UTC, which is a
            // label for the session rather than the instant it closed.
            date: self.date.date_naive(),
            open: self.open,
            high: self.high,
            low: self.low,
            close: self.close,
            volume: self.volume,
            total_return_close: self.adj_close,
            dividend_cash: self.div_cash,
            split_factor: self.split_factor,
            split_adjusted_volume: self.adj_volume,
        }
    }
}

fn body_excerpt(body: &[u8]) -> String {
    let text = String::from_utf8_lossy(body);
    let mut chars = text.chars();
    let excerpt: String = chars.by_ref().take(ERROR_BODY_LIMIT).collect();
    if chars.next().is_some() {
        format!("{excerpt}…")
    } else {
        excerpt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tickers_that_could_escape_the_url_path_are_rejected() {
        assert!(is_valid_symbol("AAPL"));
        assert!(is_valid_symbol("BRK-B"));
        assert!(is_valid_symbol("BF.B"));
        // The symbol is interpolated into a path segment.
        assert!(!is_valid_symbol("../../admin"));
        assert!(!is_valid_symbol("AAPL/prices"));
        assert!(!is_valid_symbol(""));
        assert!(!is_valid_symbol("AA PL"));
    }

    #[test]
    fn the_token_never_reaches_the_url() {
        let tiingo = Tiingo::new("secret-token").unwrap();
        assert!(!tiingo.base_url.as_str().contains("secret-token"));
    }

    #[test]
    fn plaintext_origins_are_refused_unless_they_are_loopback() {
        assert!(Tiingo::with_base_url("t", "http://api.tiingo.com").is_err());
        assert!(Tiingo::with_base_url("t", "http://127.0.0.1:8080").is_ok());
        assert!(Tiingo::with_base_url("t", "https://api.tiingo.com").is_ok());
    }
}
