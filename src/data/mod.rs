//! Historical bars, and the sources they come from.
//!
//! This is the data layer of the three the project is built from (see
//! `docs/roadmap.md`): market history for research, separate from `client`,
//! which talks to the execution venue. The separation exists because eToro is
//! a usable broker and a poor archive -- 1000 bars, no date range -- so the
//! series a strategy is tested on will not come from the venue it trades on.
//!
//! Everything here is source-agnostic. [`BarSource`] is the seam: adding a
//! vendor means implementing it, and nothing downstream changes.

pub mod tiingo;

use chrono::NaiveDate;

use crate::types::manual::Numeric;

/// What a source's `close` field actually means.
///
/// Sources disagree, silently, and the difference is invisible in a price
/// series -- which is why this is a required part of the contract rather than
/// a comment. Measured 2026-08-30: eToro's candles are [`SplitAdjusted`], while
/// Tiingo's `close` is [`AsTraded`] and its `adjClose` is [`TotalReturn`].
///
/// Getting this wrong is not a rounding error. Reading an [`AsTraded`] series
/// as though it were [`SplitAdjusted`] presents a 4:1 split as a 75%
/// single-bar drawdown, and a strategy will trade it as though it were real.
///
/// [`AsTraded`]: Self::AsTraded
/// [`SplitAdjusted`]: Self::SplitAdjusted
/// [`TotalReturn`]: Self::TotalReturn
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceBasis {
    /// Exactly as traded. A split appears as a discontinuity.
    AsTraded,
    /// Corrected for splits. Dividends are not reinvested, so this is a
    /// price-return series: holding it understates the return of a payer.
    SplitAdjusted,
    /// Corrected for splits and dividends -- a total-return series.
    TotalReturn,
}

/// One period of price history for one instrument.
///
/// `close` means whatever the producing source's [`BarSource::basis`] says it
/// means; a `Bar` on its own does not carry that. Normalising between bases is
/// deliberately a separate, explicit step rather than something a fetch does
/// quietly, so a converted series is never mistaken for an original one.
///
/// `total_return_close`, `dividend_cash` and `split_factor` are `Option`
/// because only some sources report them. Storing them rather than folding
/// them into `close` is the point: an adjusted series discards the information
/// needed to recover the unadjusted one, and which basis a strategy needs is
/// not knowable when the bar is written.
#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    /// The session this bar covers, in the exchange's local trading calendar.
    ///
    /// A date, not a timestamp: sources disagree about the instant a daily bar
    /// closes -- eToro's closes run about 0.16% below Tiingo's, consistently
    /// enough to look like a bid rather than a last trade -- and a timestamp
    /// would imply a precision that comparison does not support.
    pub date: NaiveDate,
    pub open: Numeric,
    pub high: Numeric,
    pub low: Numeric,
    pub close: Numeric,
    /// Absent where a source does not report it, which is not the same as zero.
    pub volume: Option<Numeric>,
    /// The [`PriceBasis::TotalReturn`] close, when the source carries both.
    pub total_return_close: Option<Numeric>,
    /// Dividend paid with this session as its ex-date, in the quote currency.
    pub dividend_cash: Option<Numeric>,
    /// Split ratio effective this session; `1` means no split.
    pub split_factor: Option<Numeric>,
}

/// An inclusive range of sessions to fetch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// A provider of historical bars.
///
/// Implementors return bars **ascending by date and free of duplicates**; the
/// store relies on both. A source that cannot honour a range returns what it
/// has rather than failing -- eToro serves a trailing window and cannot seek,
/// and a short series is normal (an instrument listed mid-range, a holiday, a
/// vendor's coverage starting late), not an error.
///
/// `async fn` in a trait makes this non-dyn-safe. That is deliberate: sources
/// are chosen at compile time, so generic dispatch costs nothing. Reach for
/// `Box<dyn BarSource>` only if sources ever need choosing at runtime, and add
/// `async_trait` then.
pub trait BarSource {
    /// What [`Bar::close`] means for every bar this source returns.
    ///
    /// Constant per source, and part of the contract precisely because it is
    /// not observable from the data.
    fn basis(&self) -> PriceBasis;

    /// A name for this source, for provenance in the store and in errors.
    fn name(&self) -> &'static str;

    /// Daily bars for `symbol` over `range`, ascending by date.
    fn daily_bars(
        &self,
        symbol: &str,
        range: DateRange,
    ) -> impl std::future::Future<Output = Result<Vec<Bar>, DataError>> + Send;
}

/// A failure while fetching history.
///
/// Separate from [`ApiError`](crate::error::ApiError), which describes eToro
/// requests specifically. A data source may not be eToro and may not even be
/// HTTP -- a local cache is a legitimate implementation.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// The source has no such instrument. Distinct from an empty result: the
    /// symbol is wrong, rather than the range being quiet.
    #[error("{source_name} does not know the symbol {symbol:?}")]
    UnknownSymbol {
        source_name: &'static str,
        symbol: String,
    },

    /// The request failed. Retryable in the way transport failures usually are.
    #[error("{source_name} request failed: {detail}")]
    Transport {
        source_name: &'static str,
        detail: String,
    },

    /// Credentials were rejected. Split from [`Self::Http`] because the fix is
    /// configuration, and retrying cannot help.
    #[error("{source_name} rejected the credentials: {detail}")]
    Unauthorized {
        source_name: &'static str,
        detail: String,
    },

    /// Any other non-success status. `status` is typed so a caller can branch
    /// on `is_server_error()` without a variant per code.
    #[error("{source_name} returned {status}: {detail}")]
    Http {
        source_name: &'static str,
        status: reqwest::StatusCode,
        detail: String,
    },

    /// A response arrived but did not match what the source documents --
    /// the data-layer counterpart of a decode failure.
    #[error("{source_name} returned an unexpected response: {detail}")]
    Malformed {
        source_name: &'static str,
        detail: String,
    },

    /// The caller asked for something the source cannot express, so nothing
    /// was sent.
    #[error("request not sent to {source_name}: {detail}")]
    InvalidRequest {
        source_name: &'static str,
        detail: String,
    },
}

impl DateRange {
    /// Fails rather than silently returning nothing when the bounds are
    /// inverted, which is nearly always a caller bug.
    pub fn new(start: NaiveDate, end: NaiveDate) -> Result<Self, &'static str> {
        if start > end {
            return Err("date range starts after it ends");
        }
        Ok(Self { start, end })
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        self.start <= date && date <= self.end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(text: &str) -> NaiveDate {
        text.parse().unwrap()
    }

    #[test]
    fn an_inverted_range_is_rejected_rather_than_returning_nothing() {
        // Silently yielding an empty series would look like "no data for that
        // period" instead of "the arguments are the wrong way round".
        assert!(DateRange::new(date("2026-01-31"), date("2026-01-01")).is_err());
    }

    #[test]
    fn a_single_day_range_is_valid_and_contains_that_day() {
        let range = DateRange::new(date("2026-01-15"), date("2026-01-15")).unwrap();
        assert!(range.contains(date("2026-01-15")));
    }

    #[test]
    fn range_bounds_are_inclusive_at_both_ends() {
        let range = DateRange::new(date("2026-01-01"), date("2026-01-31")).unwrap();
        assert!(range.contains(date("2026-01-01")));
        assert!(range.contains(date("2026-01-31")));
        assert!(!range.contains(date("2025-12-31")));
        assert!(!range.contains(date("2026-02-01")));
    }
}
