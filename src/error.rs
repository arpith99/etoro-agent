//! Typed errors for the eToro client.
//!
//! Two enums, because the call sites are disjoint: [`ClientError`] can only
//! happen while constructing a client (fix the configuration and restart) and
//! [`ApiError`] can only happen while making a request (which is where retry
//! decisions live).
//!
//! The variants are deliberately fewer than the number of places the client can
//! fail. They are shaped by the decisions a caller makes -- retry or not, wait
//! how long, is this a credentials problem, is this spec drift -- rather than by
//! the code paths that produce them. Where a decision needs more nuance than the
//! variant carries, it lives in a method ([`ApiError::is_retryable`]) so that the
//! branch exists in exactly one place instead of at every call site.

use std::time::Duration;

use reqwest::StatusCode;

/// Failure while constructing an HTTP client that carries credentials.
///
/// Used by [`EtoroClient`](crate::client::EtoroClient) and by the credentialed
/// data sources in [`crate::data`]: the failures are identical, and splitting
/// them would duplicate four variants to no purpose.
///
/// Nothing here is retryable: every variant means the configuration is wrong.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    /// A credential contained bytes that cannot go in an HTTP header.
    ///
    /// Carries *which* key was at fault, which the previous string-based errors
    /// could not distinguish.
    #[error("{header} is not a valid HTTP header value")]
    InvalidCredential {
        header: &'static str,
        #[source]
        source: reqwest::header::InvalidHeaderValue,
    },

    #[error("invalid eToro API base URL {url:?}")]
    InvalidBaseUrl {
        url: String,
        #[source]
        source: url::ParseError,
    },

    /// Distinct from [`Self::InvalidBaseUrl`]: the URL parsed fine and we are
    /// *refusing* it, because every request carries credentials and plaintext
    /// is only tolerable where the bytes cannot leave the machine.
    #[error("refusing to send API credentials over non-HTTPS base URL {url}")]
    InsecureBaseUrl { url: String },

    #[error("could not build the HTTP client")]
    Build(#[from] reqwest::Error),
}

/// The machine-readable failure envelope eToro attaches to `isSucceeded: false`
/// responses. Kept as plain strings so this module does not depend on generated
/// types, which are regenerated from the spec snapshot.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ExceptionDetail {
    pub reason: Option<String>,
    pub message: Option<String>,
    pub invalid_items: Vec<String>,
}

impl std::fmt::Display for ExceptionDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut parts = Vec::new();
        if let Some(reason) = &self.reason {
            parts.push(format!("reason {reason:?}"));
        }
        if let Some(message) = &self.message {
            parts.push(format!("message {message:?}"));
        }
        if !self.invalid_items.is_empty() {
            parts.push(format!("invalid items {:?}", self.invalid_items));
        }
        if parts.is_empty() {
            f.write_str("empty exception object")
        } else {
            f.write_str(&parts.join(", "))
        }
    }
}

/// A failed API request, with the context needed to report it.
///
/// `request_id` and `url` live here rather than on every variant: they apply to
/// every request-path failure, so hoisting them means each [`ApiErrorKind`]
/// describes only what actually went wrong and `Display` renders the context
/// once.
///
/// `kind` is intentionally **not** marked `#[source]`. Its text is already part
/// of this type's `Display`, so exposing it as a source as well would make
/// chain-walking formatters (`anyhow`'s `{:#}`, for one) print it twice. Callers
/// that need the underlying `reqwest`/`serde_json` error match on `kind`.
///
/// For the same reason the wrapped `reqwest`/`serde_json` errors are inlined
/// into their variants' `Display` rather than left to `source()`: without a
/// chain to walk, `serde`'s "missing field `gcid`" -- the only actionable part
/// of a decode failure -- would otherwise never reach the reader.
/// The HTTP method a failed request used.
///
/// An enum rather than a `&'static str` for two reasons: only these are ever
/// sent, and a fat pointer would push [`ApiError`] past the size at which
/// returning it by value starts costing more than it is worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Get => "GET",
            Self::Post => "POST",
        })
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{kind} ({method} {url}, request ID {request_id})")]
pub struct ApiError {
    /// The `x-request-id` sent with the request. A `Uuid` rather than a
    /// `String` because it always is one, and because it is the handle a write
    /// is recovered by -- a type that can hold anything else would be inviting
    /// something else.
    pub request_id: uuid::Uuid,
    /// So a failed write is never reported as a failed read. This matters more
    /// than it looks: "did that order go out?" is the first question after a
    /// write fails, and a message saying `GET` answers it wrongly.
    pub method: Method,
    pub url: String,
    pub kind: ApiErrorKind,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiErrorKind {
    /// The request never completed: DNS, connection, TLS, timeout, or a failure
    /// part-way through reading the body.
    #[error("request failed: {0}")]
    Transport(#[from] reqwest::Error),

    /// HTTP 429. Split out of [`Self::Http`] because it is the only status that
    /// carries data the caller needs -- how long to wait -- and the only one
    /// where the correct action is to wait exactly that long.
    #[error("rate limited{}", match .retry_after {
        Some(after) => format!(", retry after {}s", after.as_secs()),
        None => String::new(),
    })]
    RateLimited { retry_after: Option<Duration> },

    /// HTTP 403. eToro returns this -- not 401 -- when a key lacks the scope for
    /// an endpoint, which is surprising often enough to be worth saying plainly
    /// rather than leaving the reader to decode a status code.
    #[error("forbidden - the API key may lack the scope for this endpoint: {body}")]
    Forbidden { body: String },

    /// Any other non-2xx status. `status` is typed so callers can branch
    /// (`status.is_server_error()`) without needing a variant per code.
    #[error("API returned {status}: {body}")]
    Http { status: StatusCode, body: String },

    /// A 2xx body that does not match the generated types. Means the spec
    /// snapshot has drifted from reality, not that the request went wrong.
    #[error("could not decode response body: {0}")]
    Decode(#[from] serde_json::Error),

    /// A 2xx body in which the API deliberately reported failure.
    #[error("API-level failure ({})", match .detail {
        Some(detail) => detail.to_string(),
        None => "no exception details".to_owned(),
    })]
    ApiFailure { detail: Option<ExceptionDetail> },

    /// A 2xx body that decoded but violates an envelope invariant -- a required
    /// field absent, or a status the API itself reports as non-success.
    #[error("response did not match the expected shape: {detail}")]
    Malformed { detail: String },

    /// The caller asked for something the API cannot express, so nothing was
    /// sent. Distinct from [`Self::Http`]: no request was made and no
    /// rate-limit budget was spent. Not retryable -- fix the call.
    #[error("request not sent: {detail}")]
    InvalidRequest { detail: String },

    /// The credentials do not grant access to the environment the client was
    /// built for.
    ///
    /// Caught deliberately at startup rather than left to surface as a 403 on
    /// the first write, because the first write is the thing you least want to
    /// be debugging. eToro scopes each key to one environment, so a key that
    /// works perfectly for reads can still be the wrong key entirely.
    #[error(
        "credentials do not grant {declared} access; the token's environment scopes are [{granted}]"
    )]
    EnvironmentMismatch {
        declared: &'static str,
        granted: String,
    },

    /// Only reachable from a malformed hardcoded path, i.e. a bug in this crate
    /// rather than anything the API did.
    #[error("invalid API path {path:?}")]
    InvalidPath { path: String },
}

impl ApiError {
    /// Whether retrying the identical request could plausibly succeed.
    ///
    /// The point of the whole taxonomy: a retry policy asks this instead of
    /// matching on status codes or error strings, so the rule lives in one place.
    /// Deliberately conservative -- a 4xx, a decode failure or a malformed
    /// envelope will fail the same way every time, and retrying a scope problem
    /// just burns rate-limit budget.
    pub fn is_retryable(&self) -> bool {
        match &self.kind {
            ApiErrorKind::Transport(error) => error.is_timeout() || error.is_connect(),
            ApiErrorKind::RateLimited { .. } => true,
            ApiErrorKind::Http { status, .. } => status.is_server_error(),
            ApiErrorKind::Forbidden { .. }
            | ApiErrorKind::Decode(_)
            | ApiErrorKind::ApiFailure { .. }
            | ApiErrorKind::Malformed { .. }
            | ApiErrorKind::InvalidRequest { .. }
            | ApiErrorKind::EnvironmentMismatch { .. }
            | ApiErrorKind::InvalidPath { .. } => false,
        }
    }

    /// How long the API asked us to wait, when it said so.
    pub fn retry_after(&self) -> Option<Duration> {
        match &self.kind {
            ApiErrorKind::RateLimited { retry_after } => *retry_after,
            _ => None,
        }
    }

    /// The HTTP status, for failures that had one.
    pub fn status(&self) -> Option<StatusCode> {
        match &self.kind {
            ApiErrorKind::RateLimited { .. } => Some(StatusCode::TOO_MANY_REQUESTS),
            ApiErrorKind::Forbidden { .. } => Some(StatusCode::FORBIDDEN),
            ApiErrorKind::Http { status, .. } => Some(*status),
            _ => None,
        }
    }
}
