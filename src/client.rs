use std::{net::IpAddr, time::Duration};

use reqwest::{StatusCode, Url, header};
use serde::de::DeserializeOwned;

use crate::error::{ApiError, ApiErrorKind, ClientError, ExceptionDetail};
use crate::types::{
    // The exception envelope is a nested type generated from an inline object,
    // not a component schema, so it has no tag facade to be re-exported from.
    components::WatchlistsResponseException,
    tags::{
        identity::MeResponse,
        market_data::{InstrumentSearchResponse, LiveRatesResponse},
        trading_real::PortfolioResponse,
        watchlists::WatchlistsResponse,
    },
};
const BASE_URL: &str = "https://public-api.etoro.com";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);
const ERROR_BODY_LIMIT: usize = 512;
/// `maxItems` on the `instrumentIds` query parameter of the live-rates
/// endpoint.
const MAX_RATE_INSTRUMENTS: usize = 100;
/// Projection for symbol resolution. `fields` is documented as required and is
/// not enforced, but omitting it returns ~50 undocumented analytic fields and
/// nulls the ones we actually want, so it is always sent.
const SYMBOL_FIELDS: &str = "instrumentId,internalSymbolFull,displayname";

#[derive(Clone)]
pub struct EtoroClient {
    http: reqwest::Client,
    base_url: Url,
}

impl EtoroClient {
    pub fn new(api_key: &str, user_key: &str) -> Result<Self, ClientError> {
        Self::with_base_url(api_key, user_key, BASE_URL)
    }

    /// Constructs a client for a custom API origin.
    ///
    /// This is primarily useful for contract tests and API-compatible sandbox
    /// environments. Authentication headers are configured exactly as in
    /// [`Self::new`]. Because those headers carry credentials, only `https`
    /// origins are accepted, except for loopback hosts (`localhost`,
    /// `127.0.0.0/8`, `::1`) where plain `http` is allowed for local mocks.
    pub fn with_base_url(
        api_key: &str,
        user_key: &str,
        base_url: &str,
    ) -> Result<Self, ClientError> {
        let mut headers = header::HeaderMap::new();

        let sensitive = |header: &'static str, value: &str| -> Result<_, ClientError> {
            let mut value = header::HeaderValue::from_str(value)
                .map_err(|source| ClientError::InvalidCredential { header, source })?;
            value.set_sensitive(true);
            Ok(value)
        };
        headers.insert("x-api-key", sensitive("x-api-key", api_key)?);
        headers.insert("x-user-key", sensitive("x-user-key", user_key)?);

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

    pub async fn watchlists(&self) -> Result<WatchlistsResponse, ApiError> {
        let (response, ctx): (WatchlistsResponse, _) =
            self.get_json("api/v1/watchlists", &[]).await?;

        match response.is_succeeded {
            Some(true) => {}
            // The API deliberately said no, and told us why.
            Some(false) => {
                return Err(ctx.error(ApiErrorKind::ApiFailure {
                    detail: response.exception.as_ref().map(exception_detail),
                }));
            }
            // Absent is not the same as false: it means the response is not the
            // shape we believe it is, which is a different bug.
            None => {
                return Err(ctx.malformed("watchlists response omitted isSucceeded"));
            }
        }
        let Some(status) = response.status else {
            return Err(ctx.malformed("watchlists response omitted status"));
        };
        if !(200..300).contains(&status) {
            return Err(ctx.malformed(format!("watchlists response reported API status {status}")));
        }

        Ok(response)
    }

    pub async fn portfolio(&self) -> Result<PortfolioResponse, ApiError> {
        let (response, ctx): (PortfolioResponse, _) =
            self.get_json("api/v1/trading/info/portfolio", &[]).await?;
        if response.client_portfolio.is_none() {
            return Err(ctx.malformed("portfolio response omitted clientPortfolio"));
        }
        Ok(response)
    }

    /// The authenticated user's profile.
    ///
    /// No envelope validation: upstream marks `gcid`, `realCid`, `demoCid`,
    /// `username` and `scopes` required, so they decode as plain values and
    /// serde rejects a response missing any of them before this returns. The
    /// check lives in the type rather than here.
    pub async fn me(&self) -> Result<MeResponse, ApiError> {
        let (response, _ctx): (MeResponse, _) = self.get_json("api/v1/me", &[]).await?;
        Ok(response)
    }

    /// Live bid and ask for up to [`MAX_RATE_INSTRUMENTS`] instruments.
    ///
    /// An empty slice returns an empty response without making a request:
    /// there is nothing to ask for, and this endpoint draws on a 120/60s pool
    /// shared with six other market-data endpoints.
    ///
    /// No envelope validation. `rates` decodes to an empty `Vec` when absent,
    /// and the API omits instruments it does not recognise rather than
    /// reporting an error -- so a response shorter than the request means some
    /// IDs are unknown, not that the response is malformed. Callers that care
    /// must check for themselves; note the response field is `instrumentID`,
    /// capitalised differently from the `instrumentId` used elsewhere.
    pub async fn rates(&self, instrument_ids: &[i64]) -> Result<LiveRatesResponse, ApiError> {
        const PATH: &str = "api/v1/market-data/instruments/rates";

        if instrument_ids.is_empty() {
            return Ok(LiveRatesResponse::default());
        }
        if instrument_ids.len() > MAX_RATE_INSTRUMENTS {
            return Err(self.not_sent(
                PATH,
                format!(
                    "instrumentIds accepts at most {MAX_RATE_INSTRUMENTS} IDs, got {}",
                    instrument_ids.len()
                ),
            ));
        }

        // The spec declares this parameter `style: form, explode: false`: one
        // key, comma-joined, not a repeated key. That is this endpoint's
        // serialization contract rather than a property of HTTP, which is why
        // the joining happens here and `get_json` never sees a list.
        let ids = instrument_ids
            .iter()
            .map(i64::to_string)
            .collect::<Vec<_>>()
            .join(",");
        let (response, _ctx): (LiveRatesResponse, _) =
            self.get_json(PATH, &[("instrumentIds", &ids)]).await?;
        Ok(response)
    }

    /// Searches the instrument catalogue.
    ///
    /// `filters` are `Instrument` field names and values -- the endpoint both
    /// projects and filters on its own response schema, so there is no fixed
    /// parameter list to model.
    ///
    /// **A filter the API does not recognise is ignored, not rejected.** A
    /// misspelled filter name therefore returns the unfiltered first page
    /// rather than an error, and this method passes that straight through.
    /// Callers must confirm the results are what they asked for;
    /// [`Self::resolve_symbol`] shows the shape that takes.
    pub async fn search(
        &self,
        filters: &[(&str, &str)],
    ) -> Result<InstrumentSearchResponse, ApiError> {
        // Observed live: this endpoint emits `instrumentId` twice inside a
        // single item, which serde's derived `Deserialize` rejects outright.
        // `serde_json::Value` accepts duplicate keys (the last one wins), so
        // decoding in two steps tolerates the quirk here without loosening
        // every other endpoint -- strictness elsewhere is what surfaces spec
        // drift. The round trip preserves arbitrary precision, so `Numeric`
        // fields are unaffected.
        let (value, ctx): (serde_json::Value, _) =
            self.get_json("api/v1/market-data/search", filters).await?;
        serde_json::from_value(value).map_err(|source| ctx.error(ApiErrorKind::Decode(source)))
    }

    /// Resolves a ticker symbol to its eToro instrument ID.
    ///
    /// Returns `Ok(None)` when nothing matches: a well-formed query with no
    /// result is not a failure, and conflating it with one would make "no such
    /// symbol" indistinguishable from "the request broke".
    ///
    /// The exact-match check is not redundant with the filter we send. Because
    /// unrecognised filters are ignored, a request that should have narrowed to
    /// one instrument can come back as an arbitrary first page -- so the match
    /// is re-established here rather than assumed. The failure mode is closed:
    /// if the server ignores the filter and the symbol is not on the page that
    /// comes back, this reports `None` rather than the wrong instrument.
    ///
    /// Matching is ASCII case-insensitive, so `aapl` resolves as `AAPL`.
    pub async fn resolve_symbol(&self, symbol: &str) -> Result<Option<i64>, ApiError> {
        let response = self
            .search(&[("internalSymbolFull", symbol), ("fields", SYMBOL_FIELDS)])
            .await?;

        Ok(response
            .items
            .iter()
            // The catalogue carries a system-aggregate pseudo-instrument with a
            // negative ID (-100000, crypto-market-wide statistics). It is not
            // tradable and must never resolve to something a caller could send
            // to an order endpoint.
            .filter(|item| item.instrument_id.is_some_and(|id| id > 0))
            .find(|item| {
                item.internal_symbol_full
                    .as_deref()
                    .is_some_and(|found| found.eq_ignore_ascii_case(symbol))
            })
            .and_then(|item| item.instrument_id))
    }

    /// Resolves a path and query into the request URL and its context.
    ///
    /// Both the URL and the request ID are built here so that a request the
    /// client refuses to send is described exactly like one that went out --
    /// and, in particular, so that an unjoinable path reports `InvalidPath`
    /// from every route rather than only from [`Self::get_json`].
    fn context(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<(Url, RequestContext), ApiError> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let mut url = self.base_url.join(path).map_err(|_| ApiError {
            request_id: request_id.clone(),
            url: self.base_url.to_string(),
            kind: ApiErrorKind::InvalidPath {
                path: path.to_owned(),
            },
        })?;
        // `query_pairs_mut` appends `?` on the way in, so an unguarded call
        // would leave every parameterless request asking for `path?`.
        if !query.is_empty() {
            url.query_pairs_mut().extend_pairs(query);
        }
        let ctx = RequestContext {
            request_id,
            url: url.to_string(),
        };
        Ok((url, ctx))
    }

    /// An error for a request this client declined to send.
    ///
    /// The request ID is generated but never transmitted; it exists so the
    /// error carries the same shape as one that did reach the API. The `kind`
    /// says "request not sent" precisely because the surrounding `Display`
    /// would otherwise read as though it had.
    fn not_sent(&self, path: &str, detail: impl Into<String>) -> ApiError {
        match self.context(path, &[]) {
            Ok((_, ctx)) => ctx.error(ApiErrorKind::InvalidRequest {
                detail: detail.into(),
            }),
            // The path itself is unusable, which is the more basic problem.
            Err(error) => error,
        }
    }

    async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<(T, RequestContext), ApiError> {
        let (url, ctx) = self.context(path, query)?;

        let response = self
            .http
            .get(url.clone())
            .header("x-request-id", &ctx.request_id)
            .send()
            .await
            .map_err(|source| ctx.error(ApiErrorKind::Transport(source)))?;

        let status = response.status();
        // Headers must be read before `bytes()` consumes the response, or
        // `retry_after` could only ever be None.
        let retry_after = response
            .headers()
            .get(header::RETRY_AFTER)
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs);

        let body = response
            .bytes()
            .await
            .map_err(|source| ctx.error(ApiErrorKind::Transport(source)))?;

        if !status.is_success() {
            let body = body_excerpt(&body);
            return Err(ctx.error(match status {
                StatusCode::TOO_MANY_REQUESTS => ApiErrorKind::RateLimited { retry_after },
                StatusCode::FORBIDDEN => ApiErrorKind::Forbidden { body },
                status => ApiErrorKind::Http { status, body },
            }));
        }

        // The serde error itself says *why* (syntax error, unknown enum variant,
        // wrong type, ...); the variant only says *where*. Calling this "invalid
        // JSON" would misdescribe the common case of a valid body that no
        // longer matches our generated types.
        let value = serde_json::from_slice(&body)
            .map_err(|source| ctx.error(ApiErrorKind::Decode(source)))?;
        Ok((value, ctx))
    }
}

/// Per-request context, so endpoint methods can raise envelope errors carrying
/// the same request ID and URL that `get_json` would have used.
struct RequestContext {
    request_id: String,
    url: String,
}

impl RequestContext {
    fn error(&self, kind: ApiErrorKind) -> ApiError {
        ApiError {
            request_id: self.request_id.clone(),
            url: self.url.clone(),
            kind,
        }
    }

    fn malformed(&self, detail: impl Into<String>) -> ApiError {
        self.error(ApiErrorKind::Malformed {
            detail: detail.into(),
        })
    }
}

/// True for hosts where plaintext HTTP cannot leave the machine.
pub(crate) fn is_loopback(url: &Url) -> bool {
    url.host_str().is_some_and(|host| {
        host.eq_ignore_ascii_case("localhost")
            || host
                .trim_start_matches('[')
                .trim_end_matches(']')
                .parse::<IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    })
}

/// Converts the generated exception envelope into the error type's own shape.
///
/// Keeping [`ExceptionDetail`] free of generated types means the error module
/// does not move every time the schema snapshot is refreshed.
fn exception_detail(exception: &WatchlistsResponseException) -> ExceptionDetail {
    ExceptionDetail {
        reason: exception.reason.clone(),
        message: exception.message.clone(),
        invalid_items: exception.invalid_items.clone(),
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
