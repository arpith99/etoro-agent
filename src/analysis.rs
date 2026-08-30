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

impl Session {
    /// The close-to-close return, which the two parts compound to exactly.
    pub fn total(&self) -> f64 {
        (1.0 + self.overnight) * (1.0 + self.intraday) - 1.0
    }
}

/// Sessions in a trading year, for annualising.
///
/// The conventional US equity figure. Both halves get the same divisor: a year
/// contains 252 overnight moves and 252 sessions.
pub const TRADING_YEAR: f64 = 252.0;

/// Summary statistics for one side of the split.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SideStats {
    pub median: f64,
    pub mean: f64,
    /// Compounded, not summed: what holding only this part would have returned.
    pub compounded: f64,
    /// Compound annual growth rate, so windows of different lengths compare.
    pub annualised: f64,
    /// Sample standard deviation of session returns, scaled by `sqrt(252)`.
    pub volatility: f64,
}

impl SideStats {
    fn from(returns: &[f64], sessions: f64) -> Self {
        let mean = returns.iter().sum::<f64>() / returns.len() as f64;
        let compounded = returns.iter().fold(1.0, |acc, r| acc * (1.0 + r)) - 1.0;

        // A total loss cannot be annualised: a real root of a non-positive
        // growth factor does not exist, so report the loss rather than NaN.
        let growth = 1.0 + compounded;
        let annualised = if growth <= 0.0 {
            -1.0
        } else {
            growth.powf(TRADING_YEAR / sessions) - 1.0
        };

        let variance = if returns.len() > 1 {
            returns.iter().map(|r| (r - mean).powi(2)).sum::<f64>() / (returns.len() - 1) as f64
        } else {
            0.0
        };

        Self {
            median: median(returns),
            mean,
            compounded,
            annualised,
            volatility: variance.sqrt() * TRADING_YEAR.sqrt(),
        }
    }

    /// Annualised return per unit of annualised volatility.
    ///
    /// **Not a Sharpe ratio**: nothing is subtracted for the risk-free rate, so
    /// it overstates by roughly `risk_free / volatility`. It is here to compare
    /// the two sides against each other, where the omission affects both.
    pub fn return_over_vol(&self) -> f64 {
        if self.volatility > 0.0 {
            self.annualised / self.volatility
        } else {
            f64::NAN
        }
    }
}

/// The decomposition over a whole series.
#[derive(Debug, Clone, PartialEq)]
pub struct GapStats {
    pub sessions: usize,
    pub years: f64,
    pub overnight: SideStats,
    pub intraday: SideStats,
    /// The whole close-to-close return over the same window.
    ///
    /// `(1 + overnight) * (1 + intraday) == 1 + total`, exactly, because the
    /// opening prices cancel. The decomposition is an identity, not an
    /// approximation.
    pub total: SideStats,
    /// Pearson correlation between the two sides, session by session.
    ///
    /// This is what separates two series with identical row statistics.
    /// Negative means gaps partly reverse during the session that follows;
    /// positive means they continue; near zero means the halves are
    /// independent, and total variance is then just their sum.
    ///
    /// `NaN` when either side never moves, since a constant cannot correlate
    /// with anything.
    pub correlation: f64,
    /// Sessions with the largest overnight moves either way, largest first.
    pub largest: Vec<Session>,
}

/// Pearson correlation coefficient.
fn correlation(xs: &[f64], ys: &[f64]) -> f64 {
    if xs.len() != ys.len() || xs.len() < 2 {
        return f64::NAN;
    }
    let n = xs.len() as f64;
    let (mean_x, mean_y) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let mut covariance = 0.0;
    let (mut var_x, mut var_y) = (0.0, 0.0);
    for (x, y) in xs.iter().zip(ys) {
        let (dx, dy) = (x - mean_x, y - mean_y);
        covariance += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }
    // A side that never moves has no variance to share.
    if var_x <= 0.0 || var_y <= 0.0 {
        return f64::NAN;
    }
    covariance / (var_x * var_y).sqrt()
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

    let count = sessions.len() as f64;
    let overnight: Vec<f64> = sessions.iter().map(|s| s.overnight).collect();
    let intraday: Vec<f64> = sessions.iter().map(|s| s.intraday).collect();
    let total: Vec<f64> = sessions.iter().map(Session::total).collect();

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
        years: count / TRADING_YEAR,
        overnight: SideStats::from(&overnight, count),
        intraday: SideStats::from(&intraday, count),
        total: SideStats::from(&total, count),
        correlation: correlation(&overnight, &intraday),
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
        let vol = |v: f64| format!("{:.1}%", v * 100.0);
        let ratio = |v: f64| {
            if v.is_finite() {
                format!("{v:.2}")
            } else {
                "-".to_owned()
            }
        };
        let row = |name: &str, side: &SideStats| {
            format!(
                "{:<11}{:>9}{:>9}{:>10}{:>9}{:>9}{:>13}\n",
                name,
                pct(side.median),
                pct(side.mean),
                pct(side.annualised),
                vol(side.volatility),
                ratio(side.return_over_vol()),
                pct(side.compounded),
            )
        };

        let mut out = format!(
            "{:<11}{:>9}{:>9}{:>10}{:>9}{:>9}{:>13}\n",
            "", "median", "mean", "annual", "vol", "ret/vol", "compounded"
        );
        out.push_str(&row("overnight", &self.overnight));
        out.push_str(&row("intraday", &self.intraday));
        out.push_str(&format!("{}\n", "-".repeat(70)));
        out.push_str(&row("total", &self.total));
        out.push_str(&format!(
            "\n{} sessions, {:.1} years. ret/vol is not a Sharpe ratio: nothing is\n\
             subtracted for the risk-free rate, so all three rows overstate equally.\n",
            self.sessions, self.years
        ));

        if self.correlation.is_finite() {
            // Without this, two series with identical rows above can behave
            // completely differently: independent halves add their variances,
            // offsetting ones cancel part of it.
            let reading = if self.correlation < -0.1 {
                "gaps partly reverse during the session that follows"
            } else if self.correlation > 0.1 {
                "gaps tend to continue during the session that follows"
            } else {
                "the two halves move independently, so total variance is their sum"
            };
            out.push_str(&format!(
                "overnight/intraday correlation {:+.2} - {reading}\n",
                self.correlation
            ));
        }

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
            (stats.total.compounded - expected).abs() < 1e-12,
            "{} vs {expected}",
            stats.total.compounded
        );
        let recombined =
            (1.0 + stats.overnight.compounded) * (1.0 + stats.intraday.compounded) - 1.0;
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
        assert!((stats.overnight.compounded - 0.05).abs() < 1e-12);
        assert!((stats.intraday.compounded - (102.0 / 105.0 - 1.0)).abs() < 1e-12);
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
        assert!((stats.total.compounded - 0.1).abs() < 1e-12);
    }

    #[test]
    fn annualising_scales_by_the_length_of_the_window() {
        // One session of +1%: annualised is that compounded 252 times.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "101", "101"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        let expected = 1.01_f64.powf(252.0) - 1.0;
        assert!((stats.overnight.annualised - expected).abs() < 1e-9);
        assert!((stats.years - 1.0 / 252.0).abs() < 1e-12);
    }

    #[test]
    fn a_total_loss_reports_minus_one_rather_than_nan() {
        // Annualising needs a real root of the growth factor, and a wipe-out
        // leaves none.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "0.0001", "0.0001"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert!(stats.overnight.annualised.is_finite());
    }

    #[test]
    fn volatility_is_zero_for_a_perfectly_steady_series_and_positive_otherwise() {
        let steady = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "100", "100"),
            bar("2026-08-05", "100", "100"),
        ];
        assert_eq!(
            gaps(&steady, SeriesChoice::Reported, 3)
                .unwrap()
                .overnight
                .volatility,
            0.0
        );

        let choppy = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "95", "95"),
        ];
        assert!(
            gaps(&choppy, SeriesChoice::Reported, 3)
                .unwrap()
                .overnight
                .volatility
                > 0.0
        );
    }

    #[test]
    fn a_positive_mean_can_still_compound_to_a_loss() {
        // Volatility drag, which is why the report shows both. +50% then -40%
        // averages +5% a session and ends down 10%.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "150", "150"),
            bar("2026-08-05", "90", "90"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert!(stats.overnight.mean > 0.0, "mean should be positive");
        assert!(
            stats.overnight.compounded < 0.0,
            "compounded should be a loss"
        );
    }

    #[test]
    fn correlation_detects_gaps_that_reverse() {
        // Every gap is undone by the session that follows.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "100"),
            bar("2026-08-05", "90", "100"),
            bar("2026-08-06", "105", "100"),
            bar("2026-08-07", "95", "100"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert!(
            stats.correlation < -0.9,
            "expected strong reversal, got {}",
            stats.correlation
        );
    }

    #[test]
    fn correlation_detects_gaps_that_continue() {
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "121"),
            bar("2026-08-05", "108", "97"),
            bar("2026-08-06", "115", "126"),
            bar("2026-08-07", "113", "101"),
        ];
        let stats = gaps(&bars, SeriesChoice::Reported, 3).unwrap();
        assert!(
            stats.correlation > 0.9,
            "expected strong continuation, got {}",
            stats.correlation
        );
    }

    #[test]
    fn a_side_that_never_moves_has_no_correlation_to_report() {
        // Constant intraday: no variance, so nothing to correlate with.
        let bars = [
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
            bar("2026-08-05", "90", "90"),
        ];
        assert!(
            gaps(&bars, SeriesChoice::Reported, 3)
                .unwrap()
                .correlation
                .is_nan()
        );
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
        assert!(stats.total.compounded.is_finite());
    }
}
