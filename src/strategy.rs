//! Concrete strategies, kept apart from the engine that runs them.
//!
//! Everything here is scaffolding rather than an edge. The point of a dual
//! moving-average crossover is that it is well understood, easy to check by
//! hand, and long since arbitraged away — which makes it a good instrument for
//! proving the harness reports honestly, and a bad one for making money. A
//! real idea should land on a harness that has already been shown to say
//! "this lost" when a strategy lost.

use crate::backtest::{Backtest, BacktestError, CostModel, FillPrice, Strategy, run};
use crate::data::{Bar, SeriesChoice};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StrategyError {
    #[error("moving-average windows must span at least one bar, got fast {fast} and slow {slow}")]
    EmptyWindow { fast: usize, slow: usize },

    /// Equal windows are rejected too: two identical averages cross only on
    /// exact ties, so the rule would be flat forever and look like a bug in
    /// the engine rather than a nonsensical parameter pair.
    #[error("the fast window must be shorter than the slow one, got fast {fast} and slow {slow}")]
    NotOrdered { fast: usize, slow: usize },
}

/// Long while the fast moving average is above the slow one, flat otherwise.
///
/// Long-only and unlevered, so the target is 1 or 0 and nothing in between.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sma {
    fast: usize,
    slow: usize,
}

impl Sma {
    pub fn new(fast: usize, slow: usize) -> Result<Self, StrategyError> {
        if fast == 0 || slow == 0 {
            return Err(StrategyError::EmptyWindow { fast, slow });
        }
        if fast >= slow {
            return Err(StrategyError::NotOrdered { fast, slow });
        }
        Ok(Self { fast, slow })
    }

    pub fn fast(&self) -> usize {
        self.fast
    }

    pub fn slow(&self) -> usize {
        self.slow
    }

    /// Mean close over the last `window` bars of `history`.
    ///
    /// Computed from the tail on every call rather than carried as a running
    /// sum. That costs `O(window)` per bar instead of `O(1)`, which at daily
    /// resolution is nothing, and it buys something worth more: no assumption
    /// about *how* the engine walks the series. A running sum would be silently
    /// wrong the first time a caller replayed a prefix or skipped a bar, and
    /// the error would look like a signal.
    fn mean(history: &[Bar], window: usize, series: SeriesChoice) -> f64 {
        let tail = &history[history.len() - window..];
        tail.iter().map(|bar| bar.ohlc(series)[3]).sum::<f64>() / window as f64
    }
}

impl Strategy for Sma {
    fn name(&self) -> String {
        format!("sma {}/{}", self.fast, self.slow)
    }

    fn target(&mut self, history: &[Bar], series: SeriesChoice) -> f64 {
        // Flat until the slow window is full. A mean over fewer bars than
        // asked for is a different and noisier statistic, and it is noisiest
        // exactly at the start of the series -- where it would otherwise set
        // the opening position off almost no information.
        if history.len() < self.slow {
            return 0.0;
        }
        if Self::mean(history, self.fast, series) > Self::mean(history, self.slow, series) {
            1.0
        } else {
            0.0
        }
    }
}

/// One cell of a parameter sweep.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SweepCell {
    pub fast: usize,
    pub slow: usize,
    /// `NaN` where the pair is invalid or never took a position.
    pub return_over_vol: f64,
}

/// A grid of results for the same instrument, window and cost model.
///
/// The reason to build one: a single good backtest is not evidence. What
/// distinguishes a real effect from a fitted one is whether the good result
/// sits in a *region* of decent results or alone in a field of bad ones. A
/// lone hot cell is overfitting with extra steps, and it is indistinguishable
/// from a genuine edge if you only ever look at the cell you chose.
#[derive(Debug, Clone, PartialEq)]
pub struct Sweep {
    pub fasts: Vec<usize>,
    pub slows: Vec<usize>,
    pub cells: Vec<SweepCell>,
    /// Median return-per-unit-volatility across the valid cells.
    pub median: f64,
    /// The same figure for buy-and-hold, identical in every cell.
    pub benchmark: f64,
}

/// Runs every `fast` against every `slow`, holding everything else fixed.
///
/// Pairs where `fast >= slow` are skipped rather than reported as failures:
/// they are not a parameter choice anyone made, only the half of a rectangle
/// that does not describe a crossover.
pub fn sweep_sma(
    bars: &[Bar],
    series: SeriesChoice,
    costs: &CostModel,
    fill: FillPrice,
    fasts: &[usize],
    slows: &[usize],
) -> Result<Sweep, BacktestError> {
    let mut cells = Vec::new();
    let mut benchmark = f64::NAN;

    for &fast in fasts {
        for &slow in slows {
            let Ok(mut strategy) = Sma::new(fast, slow) else {
                continue;
            };
            let result = run(bars, series, &mut strategy, costs, fill)?;
            // Identical in every cell, since the benchmark ignores the
            // strategy entirely; taken from the first run that produced one.
            benchmark = result.benchmark.return_over_vol();
            cells.push(SweepCell {
                fast,
                slow,
                return_over_vol: result.strategy.return_over_vol(),
            });
        }
    }

    let mut valid: Vec<f64> = cells
        .iter()
        .map(|cell| cell.return_over_vol)
        .filter(|value| value.is_finite())
        .collect();
    valid.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = match valid.len() {
        0 => f64::NAN,
        n if n % 2 == 1 => valid[n / 2],
        n => (valid[n / 2 - 1] + valid[n / 2]) / 2.0,
    };

    Ok(Sweep {
        fasts: fasts.to_vec(),
        slows: slows.to_vec(),
        cells,
        median,
        benchmark,
    })
}

impl Sweep {
    pub fn cell(&self, fast: usize, slow: usize) -> Option<&SweepCell> {
        self.cells
            .iter()
            .find(|cell| cell.fast == fast && cell.slow == slow)
    }

    /// The grid, rows by fast window and columns by slow.
    pub fn grid(&self) -> String {
        let mut out = format!("{:>6}", "");
        for slow in &self.slows {
            out.push_str(&format!("{slow:>8}"));
        }
        out.push('\n');

        for fast in &self.fasts {
            out.push_str(&format!("{fast:>6}"));
            for slow in &self.slows {
                out.push_str(&match self.cell(*fast, *slow) {
                    Some(cell) if cell.return_over_vol.is_finite() => {
                        format!("{:>8.2}", cell.return_over_vol)
                    }
                    // Skipped and undefined print alike: neither is a result,
                    // and distinguishing them in the grid would only invite
                    // reading the empty half as information.
                    _ => format!("{:>8}", "-"),
                });
            }
            out.push('\n');
        }

        out.push_str(&format!(
            "\nrows are the fast window, columns the slow; cells are return per unit\n\
             of volatility. median {} across {} pairs, against {} for buy & hold.\n",
            fmt(self.median),
            self.cells.len(),
            fmt(self.benchmark),
        ));
        out.push_str(
            "a single strong cell surrounded by weak ones is a fitted parameter, not\n\
             an edge; what would count is a broad region that holds up.\n",
        );
        out
    }
}

fn fmt(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.2}")
    } else {
        "-".to_owned()
    }
}

/// Runs one parameter pair and returns the full result.
pub fn backtest_sma(
    bars: &[Bar],
    series: SeriesChoice,
    costs: &CostModel,
    fill: FillPrice,
    fast: usize,
    slow: usize,
) -> Result<Backtest, SmaError> {
    let mut strategy = Sma::new(fast, slow)?;
    Ok(run(bars, series, &mut strategy, costs, fill)?)
}

/// Either the parameters were nonsense or the series was.
#[derive(Debug, thiserror::Error)]
pub enum SmaError {
    #[error(transparent)]
    Parameters(#[from] StrategyError),
    #[error(transparent)]
    Backtest(#[from] BacktestError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::manual::Numeric;

    fn bar(date: &str, close: &str) -> Bar {
        let n = |v: &str| Numeric(v.parse().unwrap());
        Bar {
            date: date.parse().unwrap(),
            open: n(close),
            high: n(close),
            low: n(close),
            close: n(close),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
            split_adjusted_volume: None,
        }
    }

    /// A run of closes on consecutive days from 2026-01-01.
    fn series(closes: &[f64]) -> Vec<Bar> {
        let start: chrono::NaiveDate = "2026-01-01".parse().unwrap();
        closes
            .iter()
            .enumerate()
            .map(|(i, close)| {
                bar(
                    &(start + chrono::Duration::days(i as i64)).to_string(),
                    &close.to_string(),
                )
            })
            .collect()
    }

    #[test]
    fn windows_that_could_not_cross_are_refused() {
        assert_eq!(
            Sma::new(0, 10),
            Err(StrategyError::EmptyWindow { fast: 0, slow: 10 })
        );
        assert_eq!(
            Sma::new(10, 0),
            Err(StrategyError::EmptyWindow { fast: 10, slow: 0 })
        );
        assert_eq!(
            Sma::new(10, 10),
            Err(StrategyError::NotOrdered { fast: 10, slow: 10 })
        );
        assert_eq!(
            Sma::new(20, 10),
            Err(StrategyError::NotOrdered { fast: 20, slow: 10 })
        );
        assert!(Sma::new(1, 2).is_ok());
    }

    #[test]
    fn the_rule_is_long_above_and_flat_below() {
        let mut sma = Sma::new(2, 4).unwrap();
        let rising = series(&[1.0, 2.0, 3.0, 4.0]);
        // Fast mean (3.5) over slow (2.5).
        assert_eq!(sma.target(&rising, SeriesChoice::Reported), 1.0);

        let falling = series(&[4.0, 3.0, 2.0, 1.0]);
        assert_eq!(sma.target(&falling, SeriesChoice::Reported), 0.0);

        // Equal means are not a crossing, so the rule stays flat.
        let flat = series(&[1.0, 1.0, 1.0, 1.0]);
        assert_eq!(sma.target(&flat, SeriesChoice::Reported), 0.0);
    }

    #[test]
    fn nothing_is_held_until_the_slow_window_is_full() {
        let mut sma = Sma::new(2, 4).unwrap();
        let rising = series(&[1.0, 2.0, 3.0, 4.0]);
        for length in 1..4 {
            assert_eq!(
                sma.target(&rising[..length], SeriesChoice::Reported),
                0.0,
                "{length} bars is fewer than the slow window"
            );
        }
        assert_eq!(sma.target(&rising, SeriesChoice::Reported), 1.0);
    }

    #[test]
    fn the_average_follows_the_series_the_engine_is_pricing() {
        // Reported closes fall while the total-return closes rise, which is
        // what an unadjusted series does across a split. A strategy reading
        // the wrong one would take exactly the opposite position.
        let mut bars = series(&[4.0, 3.0, 2.0, 1.0]);
        for (i, bar) in bars.iter_mut().enumerate() {
            bar.total_return_close = Some(Numeric(((i + 1) as f64).to_string().parse().unwrap()));
        }

        let mut sma = Sma::new(2, 4).unwrap();
        assert_eq!(sma.target(&bars, SeriesChoice::Reported), 0.0);
        assert_eq!(sma.target(&bars, SeriesChoice::TotalReturn), 1.0);
    }

    #[test]
    fn a_sweep_skips_the_half_of_the_grid_that_is_not_a_crossover() {
        let bars = series(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        let sweep = sweep_sma(
            &bars,
            SeriesChoice::Reported,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
            &[2, 4],
            &[2, 4],
        )
        .unwrap();

        // Of the four pairs only (2, 4) is a valid crossover.
        assert_eq!(sweep.cells.len(), 1);
        assert_eq!(sweep.cells[0].fast, 2);
        assert_eq!(sweep.cells[0].slow, 4);
        assert!(sweep.cell(4, 2).is_none());

        let grid = sweep.grid();
        assert!(grid.contains('-'), "the skipped pairs render as dashes");
        assert!(grid.contains("buy & hold"), "{grid}");
    }

    #[test]
    fn a_sweep_reports_the_median_across_its_valid_cells() {
        // A trending series, so the crossover holds through most of it.
        let closes: Vec<f64> = (1..=40).map(f64::from).collect();
        let sweep = sweep_sma(
            &series(&closes),
            SeriesChoice::Reported,
            &CostModel::frictionless(),
            FillPrice::NextOpen,
            &[2, 5],
            &[10, 20],
        )
        .unwrap();

        assert_eq!(sweep.cells.len(), 4);
        let mut values: Vec<f64> = sweep.cells.iter().map(|c| c.return_over_vol).collect();
        values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert!((sweep.median - (values[1] + values[2]) / 2.0).abs() < 1e-12);
        // Every cell rode the same monotone rise, so the benchmark is finite
        // and shared rather than recomputed per cell.
        assert!(sweep.benchmark.is_finite());
    }

    #[test]
    fn bad_parameters_and_a_bad_series_are_distinguishable() {
        let bars = series(&[1.0, 2.0, 3.0]);
        let costs = CostModel::frictionless();
        assert!(matches!(
            backtest_sma(
                &bars,
                SeriesChoice::Reported,
                &costs,
                FillPrice::NextOpen,
                50,
                10
            ),
            Err(SmaError::Parameters(StrategyError::NotOrdered { .. }))
        ));
        assert!(matches!(
            backtest_sma(
                &bars[..1],
                SeriesChoice::Reported,
                &costs,
                FillPrice::NextOpen,
                2,
                4
            ),
            Err(SmaError::Backtest(BacktestError::TooShort { .. }))
        ));
    }
}
