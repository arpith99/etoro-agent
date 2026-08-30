//! Statistics over a stored bar series.
//!
//! Separate from [`chart`](crate::chart), which draws; this measures. Both go
//! through [`Bar::ohlc`](crate::data::Bar::ohlc) so the split- and
//! dividend-adjustment logic exists once.

use chrono::NaiveDate;

use crate::data::{Bar, SeriesChoice};

/// A session's return split into the part earned while the market was shut and
/// the part earned while it was open.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Session {
    pub date: NaiveDate,
    /// `open / previous close - 1`. Everything that happened in the roughly
    /// seventeen hours between one session's close and the next one's opening
    /// auction: earnings released deliberately outside trading hours, macro
    /// data timed for 08:30 ET, overseas sessions, and thin extended-hours
    /// trading.
    pub overnight: f64,
    /// `close / open - 1`. The continuous session itself.
    pub intraday: f64,
}

/// The decomposition over a whole series.
#[derive(Debug, Clone, PartialEq)]
pub struct GapStats {
    pub sessions: usize,
    pub overnight_median: f64,
    pub intraday_median: f64,
    pub overnight_mean: f64,
    pub intraday_mean: f64,
    /// Compounded, not summed: what holding only that part would have returned.
    pub overnight_compounded: f64,
    pub intraday_compounded: f64,
    /// The whole close-to-close return over the same window.
    ///
    /// `(1 + overnight) * (1 + intraday) == 1 + total`, exactly, because the
    /// opening prices cancel. The decomposition is an identity rather than an
    /// approximation.
    pub total_compounded: f64,
    /// Sessions with the largest overnight moves either way, largest first.
    pub largest: Vec<Session>,
}

/// Splits each session's return into overnight and intraday parts.
///
/// The first bar is skipped: it has no previous close, so no overnight move.
/// Its intraday move is skipped too, which is what keeps the identity above
/// exact — both parts then cover the same window, bar 1 through the last.
///
/// Prefer [`SeriesChoice::TotalReturn`] where the source has it. On as-traded
/// prices an ex-dividend date shows up as an overnight loss that no holder
/// suffered, and a split shows up as an overnight collapse.
pub fn gaps(bars: &[Bar], series: SeriesChoice, largest_count: usize) -> Option<GapStats> {
    if bars.len() < 2 {
        return None;
    }

    let mut sessions = Vec::with_capacity(bars.len() - 1);
    for pair in bars.windows(2) {
        let previous_close = pair[0].ohlc(series)[3];
        let [open, _, _, close] = pair[1].ohlc(series);
        if previous_close <= 0.0 || open <= 0.0 {
            continue;
        }
        sessions.push(Session {
            date: pair[1].date,
            overnight: open / previous_close - 1.0,
            intraday: close / open - 1.0,
        });
    }
    if sessions.is_empty() {
        return None;
    }

    let compound = |values: &[f64]| values.iter().fold(1.0, |acc, r| acc * (1.0 + r)) - 1.0;
    let mean = |values: &[f64]| values.iter().sum::<f64>() / values.len() as f64;
    let overnight: Vec<f64> = sessions.iter().map(|s| s.overnight).collect();
    let intraday: Vec<f64> = sessions.iter().map(|s| s.intraday).collect();

    let mut largest = sessions.clone();
    largest.sort_by(|a, b| {
        b.overnight
            .abs()
            .partial_cmp(&a.overnight.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    largest.truncate(largest_count);

    Some(GapStats {
        sessions: sessions.len(),
        overnight_median: median(&overnight),
        intraday_median: median(&intraday),
        overnight_mean: mean(&overnight),
        intraday_mean: mean(&intraday),
        overnight_compounded: compound(&overnight),
        intraday_compounded: compound(&intraday),
        total_compounded: (1.0 + compound(&overnight)) * (1.0 + compound(&intraday)) - 1.0,
        largest,
    })
}

fn median(values: &[f64]) -> f64 {
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    match sorted.len() {
        0 => f64::NAN,
        n if n % 2 == 1 => sorted[n / 2],
        n => (sorted[n / 2 - 1] + sorted[n / 2]) / 2.0,
    }
}

impl GapStats {
    /// A fixed-width report.
    pub fn report(&self) -> String {
        let pct = |v: f64| format!("{:+.2}%", v * 100.0);
        let mut out = format!(
            "{:<12}{:>10}{:>10}{:>14}\n",
            "", "median", "mean", "compounded"
        );
        out.push_str(&format!(
            "{:<12}{:>10}{:>10}{:>14}\n",
            "overnight",
            pct(self.overnight_median),
            pct(self.overnight_mean),
            pct(self.overnight_compounded),
        ));
        out.push_str(&format!(
            "{:<12}{:>10}{:>10}{:>14}\n",
            "intraday",
            pct(self.intraday_median),
            pct(self.intraday_mean),
            pct(self.intraday_compounded),
        ));
        out.push_str(&format!("{}\n", "-".repeat(46)));
        out.push_str(&format!(
            "{:<12}{:>10}{:>10}{:>14}\n",
            "total",
            "",
            "",
            pct(self.total_compounded)
        ));

        if !self.largest.is_empty() {
            out.push_str("\nlargest overnight moves:\n");
            for session in &self.largest {
                out.push_str(&format!(
                    "  {}  {:>8}   (session {:>8})\n",
                    session.date,
                    pct(session.overnight),
                    pct(session.intraday),
                ));
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::manual::Numeric;

    fn bar(date: &str, open: &str, close: &str) -> Bar {
        let n = |v: &str| Numeric(v.parse().unwrap());
        Bar {
            date: date.parse().unwrap(),
            open: n(open),
            high: n(close),
            low: n(open),
            close: n(close),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
            split_adjusted_volume: None,
        }
    }

    #[test]
    fn a_series_too_short_to_have_a_gap_reports_nothing() {
        assert!(gaps(&[], SeriesChoice::Reported, 3).is_none());
        assert!(
            gaps(
                &[bar("2026-08-03", "100", "101")],
                SeriesChoice::Reported,
                3
            )
            .is_none()
        );
    }

    #[test]
    fn the_two_parts_multiply_back_to_the_close_to_close_return() {
        // The identity the whole decomposition rests on: opening prices cancel.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "105", "102"),
            bar("2026-08-05", "99", "110"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();

        let expected = 110.0 / 100.0 - 1.0;
        assert!(
            (stats.total_compounded - expected).abs() < 1e-12,
            "{} vs {expected}",
            stats.total_compounded
        );
        let recombined =
            (1.0 + stats.overnight_compounded) * (1.0 + stats.intraday_compounded) - 1.0;
        assert!((recombined - expected).abs() < 1e-12);
    }

    #[test]
    fn overnight_and_intraday_are_measured_separately() {
        // Up 5% overnight, then down ~2.9% during the session.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "105", "102"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert_eq!(stats.sessions, 1);
        assert!((stats.overnight_compounded - 0.05).abs() < 1e-12);
        assert!((stats.intraday_compounded - (102.0 / 105.0 - 1.0)).abs() < 1e-12);
    }

    #[test]
    fn the_first_bar_contributes_nothing_since_it_has_no_previous_close() {
        let bars = [
            bar("2026-08-03", "50", "100"),
            bar("2026-08-04", "100", "110"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        // The first bar's own +100% session is excluded, keeping both parts
        // over the same window.
        assert_eq!(stats.sessions, 1);
        assert!((stats.total_compounded - 0.1).abs() < 1e-12);
    }

    #[test]
    fn largest_moves_are_ranked_by_size_in_either_direction() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "101", "101"), // +1% overnight
            bar("2026-08-05", "91", "91"),   // about -10% overnight
            bar("2026-08-06", "93", "93"),   // about +2% overnight
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 2).unwrap();
        assert_eq!(stats.largest.len(), 2);
        assert_eq!(stats.largest[0].date.to_string(), "2026-08-05");
        assert!(stats.largest[0].overnight < 0.0, "a fall must rank too");
    }

    #[test]
    fn a_zero_previous_close_is_skipped_rather_than_producing_infinity() {
        let bars = [
            bar("2026-08-03", "0", "0"),
            bar("2026-08-04", "100", "101"),
            bar("2026-08-05", "101", "102"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert_eq!(stats.sessions, 1);
        assert!(stats.total_compounded.is_finite());
    }
}
