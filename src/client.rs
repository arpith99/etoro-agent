use std::{net::IpAddr, time::Duration};

use anyhow::{Context, Result, bail, ensure};
use reqwest::{Url, header};
use serde::de::DeserializeOwned;

use crate::types::{
    portfolio::PortfolioResponse,
    watchlists::{WatchlistsResponse, WatchlistsResponseException},
};
const BASE_URL: &str = "https://public-api.etoro.com";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const ERROR_BODY_LIMIT: usize = 512;

#[derive(Clone)]
pub struct EtoroClient {
    http: reqwest::Client,
    base_url: Url,
}

impl EtoroClient {
    pub fn new(api_key: &str, user_key: &str) -> Result<Self> {
        Self::with_base_url(api_key, user_key, BASE_URL)
    }

    /// Constructs a client for a custom API origin.
    ///
    /// This is primarily useful for contract tests and API-compatible sandbox
    /// environments. Authentication headers are configured exactly as in
    /// [`Self::new`]. Because those headers carry credentials, only `https`
    /// origins are accepted, except for loopback hosts (`localhost`,
    /// `127.0.0.0/8`, `::1`) where plain `http` is allowed for local mocks.
    pub fn with_base_url(api_key: &str, user_key: &str, base_url: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        let sensitive = |s: &str| -> Result<header::HeaderValue> {
            let mut value = header::HeaderValue::from_str(s)?;
            value.set_sensitive(true);
            Ok(value)
        };
        headers.insert("x-api-key", sensitive(api_key)?);
        headers.insert("x-user-key", sensitive(user_key)?);

        let base_url: Url = format!("{}/", base_url.trim_end_matches('/'))
            .parse()
            .context("invalid eToro API base URL")?;
        ensure!(
            base_url.scheme() == "https" || (base_url.scheme() == "http" && is_loopback(&base_url)),
            "refusing to send API credentials over non-HTTPS base URL {base_url}"
        );
        Ok(Self {
            http: reqwest::Client::builder()
                .default_headers(headers)
                .timeout(REQUEST_TIMEOUT)
                .build()?,
            base_url,
        })
    }

    pub async fn watchlists(&self) -> Result<WatchlistsResponse> {
        let (response, request_id): (WatchlistsResponse, _) =
            self.get_json("api/v1/watchlists").await?;

        match response.is_succeeded {
            Some(true) => {}
            Some(false) => {
                let detail = describe_exception(response.exception.as_ref());
                bail!(
                    "watchlists response reported an API-level failure ({detail}; request ID {request_id})"
                )
            }
            None => bail!("watchlists response omitted isSucceeded (request ID {request_id})"),
        }
        let status = response.status.with_context(|| {
            format!("watchlists response omitted status (request ID {request_id})")
        })?;
        ensure!(
            (200..300).contains(&status),
            "watchlists response reported API status {status} (request ID {request_id})"
        );

        Ok(response)
    }

    pub async fn portfolio(&self) -> Result<PortfolioResponse> {
        let (response, request_id): (PortfolioResponse, _) =
            self.get_json("api/v1/trading/info/portfolio").await?;
        ensure!(
            response.client_portfolio.is_some(),
            "portfolio response omitted clientPortfolio (request ID {request_id})"
        );
        Ok(response)
    }

    async fn get_json<T: DeserializeOwned>(&self, path: &str) -> Result<(T, String)> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let url = self
            .base_url
            .join(path)
            .with_context(|| format!("invalid API path {path:?}"))?;

        let response = self
            .http
            .get(url.clone())
            .header("x-request-id", &request_id)
            .send()
            .await
            .with_context(|| format!("GET {url} failed (request ID {request_id})"))?;

        let status = response.status();
        let body = response.bytes().await.with_context(|| {
            format!("failed to read GET {url} response body (request ID {request_id})")
        })?;

        if !status.is_success() {
            bail!(
                "GET {url} returned {status} (request ID {request_id}): {}",
                body_excerpt(&body)
            );
        }

        // The serde error itself says *why* (syntax error, unknown enum variant,
        // wrong type, ...); the context only says *where*. Calling this "invalid
        // JSON" would misdescribe the common case of a valid body that no
        // longer matches our generated types.
        let value = serde_json::from_slice(&body).with_context(|| {
            format!("could not decode response body from GET {url} (request ID {request_id})")
        })?;
        Ok((value, request_id))
    }
}

/// True for hosts where plaintext HTTP cannot leave the machine.
fn is_loopback(url: &Url) -> bool {
    url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    })
}

/// Renders the machine-readable failure envelope eToro attaches to
/// `isSucceeded: false` responses, so callers see the reason, not just the fact.
fn describe_exception(exception: Option<&WatchlistsResponseException>) -> String {
    let Some(exception) = exception else {
        return "no exception details".to_owned();
    };
    let mut parts = Vec::new();
    if let Some(reason) = &exception.reason {
        parts.push(format!("reason {reason:?}"));
    }
    if let Some(message) = &exception.message {
        parts.push(format!("message {message:?}"));
    }
    if !exception.invalid_items.is_empty() {
        parts.push(format!("invalid items {:?}", exception.invalid_items));
    }
    if parts.is_empty() {
        "empty exception object".to_owned()
    } else {
        parts.join(", ")
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
