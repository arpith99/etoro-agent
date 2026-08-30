//! Retrying a request without turning one trade into two.
//!
//! The hazard this module exists for is specific to writes. eToro documents
//! `x-request-id` as an idempotency key and echoes it back as `referenceId`, so
//! **a retry that reuses its id is the same order and a retry that mints a new
//! one is a second order.** A timed-out submission looks identical either way
//! from here: the request may have arrived, executed, and had its response lost
//! on the way back.
//!
//! [`with_retry`] therefore generates one id and hands the *same* one to every
//! attempt. That makes correct reuse the path of least resistance rather than
//! something to remember, which matters because the consequence of forgetting
//! is a duplicate position rather than an error.
//!
//! Reads are unaffected: [`get_json`](crate::client) mints its own id per
//! request, so a read closure can ignore the argument.

use std::future::Future;
use std::time::Duration;

use crate::error::ApiError;

/// How hard to try, and how long to wait between attempts.
///
/// There is no `Default`: how many times to resend a request that might have
/// placed an order is not a decision to inherit silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetryPolicy {
    /// Total attempts including the first, so `1` means no retrying.
    pub max_attempts: u32,
    /// Wait before the second attempt; doubles each time after that.
    pub base_delay: Duration,
    /// Ceiling on any single wait, including one the API asked for.
    pub max_delay: Duration,
}

impl RetryPolicy {
    /// Send once and report whatever happens.
    ///
    /// The right policy for a write whose duplication would be worse than its
    /// failure, and the one to reach for before an order path has been watched
    /// on demo for a while.
    pub fn once() -> Self {
        Self {
            max_attempts: 1,
            base_delay: Duration::ZERO,
            max_delay: Duration::ZERO,
        }
    }

    /// Three attempts, one second apart, doubling, capped at a minute.
    ///
    /// Sized against the API's own limits rather than picked round: the
    /// execution pool refills 20 requests per 60 seconds, so a wait longer than
    /// the window buys nothing a fresh call would not, and the cap matches it.
    pub fn standard() -> Self {
        Self {
            max_attempts: 3,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        }
    }

    /// How long to wait before attempt number `next_attempt`, or `None` to give
    /// up.
    ///
    /// Pure, so the whole decision can be tested without waiting for it.
    ///
    /// A `Retry-After` from the API wins over the computed backoff, in both
    /// directions: it is the only number here that reflects what the server
    /// actually knows. It is still capped, because an hour-long header would
    /// otherwise park an unattended agent indefinitely.
    pub fn delay_before(&self, next_attempt: u32, error: &ApiError) -> Option<Duration> {
        if next_attempt > self.max_attempts || !error.is_retryable() {
            return None;
        }
        let backoff = error.retry_after().unwrap_or_else(|| {
            // next_attempt is 2 for the first retry, so this starts at base.
            let doublings = next_attempt.saturating_sub(2);
            self.base_delay
                .saturating_mul(2_u32.saturating_pow(doublings.min(16)))
        });
        Some(backoff.min(self.max_delay))
    }
}

/// Runs `operation` until it succeeds or `policy` gives up.
///
/// Every attempt receives the **same** request id — see the module docs. Pass
/// it through to any write; ignore it for a read.
///
/// ```no_run
/// # use etoro_agent::{client::EtoroClient, orders::MarketBuy,
/// #                   retry::{RetryPolicy, with_retry}};
/// # async fn example(client: &EtoroClient, order: &MarketBuy) -> anyhow::Result<()> {
/// // Retried submissions reuse one id, so at most one order exists.
/// let accepted = with_retry(&RetryPolicy::standard(), |request_id| {
///     client.place_order(order, request_id)
/// })
/// .await?;
/// # let _ = accepted;
/// # Ok(())
/// # }
/// ```
pub async fn with_retry<T, F, Fut>(policy: &RetryPolicy, operation: F) -> Result<T, ApiError>
where
    F: FnMut(uuid::Uuid) -> Fut,
    Fut: Future<Output = Result<T, ApiError>>,
{
    // Generated once, outside the loop. This line is the whole point.
    with_retry_id(policy, uuid::Uuid::new_v4(), operation).await
}

/// [`with_retry`], with the caller supplying the id.
///
/// For when something has to happen *before* the first attempt and needs the
/// id — writing an audit record, most obviously. An order has to be logged
/// before it is sent, because that is the only way the log can name an order
/// whose response never arrived; and that write should be able to abort the
/// whole operation, which it cannot do from inside the closure.
///
/// The id is still used for every attempt. Passing a fresh one per call is the
/// caller's job and their opportunity to get it wrong, which is why
/// [`with_retry`] exists and should be preferred when nothing needs the id
/// early.
pub async fn with_retry_id<T, F, Fut>(
    policy: &RetryPolicy,
    request_id: uuid::Uuid,
    mut operation: F,
) -> Result<T, ApiError>
where
    F: FnMut(uuid::Uuid) -> Fut,
    Fut: Future<Output = Result<T, ApiError>>,
{
    let mut attempt = 1;
    loop {
        let error = match operation(request_id).await {
            Ok(value) => return Ok(value),
            Err(error) => error,
        };
        match policy.delay_before(attempt + 1, &error) {
            Some(delay) => tokio::time::sleep(delay).await,
            None => return Err(error),
        }
        attempt += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};

    use super::*;
    use crate::error::{ApiErrorKind, Method};
    use reqwest::StatusCode;

    fn error(kind: ApiErrorKind) -> ApiError {
        ApiError {
            request_id: uuid::Uuid::nil(),
            method: Method::Post,
            url: "https://example.invalid/orders".to_owned(),
            kind,
        }
    }

    fn server_error() -> ApiError {
        error(ApiErrorKind::Http {
            status: StatusCode::BAD_GATEWAY,
            body: String::new(),
        })
    }

    fn rate_limited(retry_after: Option<Duration>) -> ApiError {
        error(ApiErrorKind::RateLimited { retry_after })
    }

    #[test]
    fn backoff_doubles_from_the_base_delay() {
        let policy = RetryPolicy {
            max_attempts: 5,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        };
        // Attempt 2 is the first retry, so it waits exactly the base.
        assert_eq!(
            policy.delay_before(2, &server_error()),
            Some(Duration::from_secs(1))
        );
        assert_eq!(
            policy.delay_before(3, &server_error()),
            Some(Duration::from_secs(2))
        );
        assert_eq!(
            policy.delay_before(4, &server_error()),
            Some(Duration::from_secs(4))
        );
    }

    #[test]
    fn the_apis_own_retry_after_wins_over_the_computed_backoff() {
        let policy = RetryPolicy::standard();
        // Longer than the backoff would have been...
        assert_eq!(
            policy.delay_before(2, &rate_limited(Some(Duration::from_secs(30)))),
            Some(Duration::from_secs(30))
        );
        // ...and shorter. The server knows something we do not, either way.
        assert_eq!(
            policy.delay_before(3, &rate_limited(Some(Duration::from_millis(250)))),
            Some(Duration::from_millis(250))
        );
    }

    #[test]
    fn no_single_wait_exceeds_the_ceiling() {
        let policy = RetryPolicy {
            max_attempts: 30,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        };
        // Neither an exponent that has run away...
        assert_eq!(
            policy.delay_before(20, &server_error()),
            Some(Duration::from_secs(60))
        );
        // ...nor a header that would park an unattended agent for an hour.
        assert_eq!(
            policy.delay_before(2, &rate_limited(Some(Duration::from_secs(3600)))),
            Some(Duration::from_secs(60))
        );
    }

    #[test]
    fn an_error_that_cannot_improve_is_not_retried() {
        let policy = RetryPolicy::standard();
        for kind in [
            ApiErrorKind::Forbidden {
                body: String::new(),
            },
            ApiErrorKind::Malformed {
                detail: "no".to_owned(),
            },
            ApiErrorKind::InvalidRequest {
                detail: "no".to_owned(),
            },
            ApiErrorKind::Http {
                status: StatusCode::BAD_REQUEST,
                body: String::new(),
            },
        ] {
            let error = error(kind);
            assert_eq!(policy.delay_before(2, &error), None, "{error}");
        }
    }

    #[test]
    fn attempts_are_bounded_even_for_retryable_errors() {
        let policy = RetryPolicy::standard();
        assert!(policy.delay_before(3, &server_error()).is_some());
        assert_eq!(policy.delay_before(4, &server_error()), None);
        // once() means exactly that: no second attempt, ever.
        assert_eq!(RetryPolicy::once().delay_before(2, &server_error()), None);
    }

    #[tokio::test]
    async fn every_attempt_is_given_the_same_request_id() {
        // The property the whole module exists for: a retried write must be
        // the same order, and it is the id that decides.
        let seen = std::sync::Mutex::new(Vec::new());
        let attempts = AtomicU32::new(0);
        let policy = RetryPolicy {
            max_attempts: 3,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
        };

        let result: Result<&str, _> = with_retry(&policy, |request_id| {
            seen.lock().unwrap().push(request_id);
            let attempt = attempts.fetch_add(1, Ordering::SeqCst);
            async move {
                if attempt < 2 {
                    Err(server_error())
                } else {
                    Ok("filled")
                }
            }
        })
        .await;

        assert_eq!(result.unwrap(), "filled");
        let seen = seen.into_inner().unwrap();
        assert_eq!(seen.len(), 3, "two failures then a success");
        assert!(
            seen.windows(2).all(|pair| pair[0] == pair[1]),
            "a fresh id would have made each retry a separate order: {seen:?}"
        );
        assert_ne!(seen[0], uuid::Uuid::nil());
    }

    #[tokio::test]
    async fn a_supplied_id_is_the_one_every_attempt_uses() {
        // The variant that lets an audit record be written before the send.
        let chosen = uuid::Uuid::new_v4();
        let seen = std::sync::Mutex::new(Vec::new());
        let attempts = AtomicU32::new(0);
        let policy = RetryPolicy {
            max_attempts: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
        };

        let result: Result<(), _> = with_retry_id(&policy, chosen, |request_id| {
            seen.lock().unwrap().push(request_id);
            let attempt = attempts.fetch_add(1, Ordering::SeqCst);
            async move {
                if attempt == 0 {
                    Err(server_error())
                } else {
                    Ok(())
                }
            }
        })
        .await;

        assert!(result.is_ok());
        assert_eq!(seen.into_inner().unwrap(), vec![chosen, chosen]);
    }

    #[tokio::test]
    async fn the_last_error_is_returned_once_attempts_run_out() {
        let attempts = AtomicU32::new(0);
        let policy = RetryPolicy {
            max_attempts: 2,
            base_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(1),
        };

        let result: Result<(), _> = with_retry(&policy, |_| {
            attempts.fetch_add(1, Ordering::SeqCst);
            async { Err(server_error()) }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 2, "not one, and not three");
    }

    #[tokio::test]
    async fn a_non_retryable_failure_is_not_sent_twice() {
        let attempts = AtomicU32::new(0);
        let result: Result<(), _> = with_retry(&RetryPolicy::standard(), |_| {
            attempts.fetch_add(1, Ordering::SeqCst);
            async {
                Err(error(ApiErrorKind::Forbidden {
                    body: "scope".to_owned(),
                }))
            }
        })
        .await;

        assert!(result.is_err());
        assert_eq!(attempts.load(Ordering::SeqCst), 1);
    }
}
