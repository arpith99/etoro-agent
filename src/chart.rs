//! Terminal rendering for a bar series.
//!
//! A fetch that silently returns the wrong thing looks identical to one that
//! worked -- both print a row count. Looking at the series catches what a
//! count cannot: an unadjusted split as a cliff, a vendor outage as a flat
//! stretch, a bad date filter as a gap.
//!
//! Rendering converts prices to `f64`. That is safe *here* precisely because
//! it is the one place where precision does not matter -- a character cell is
//! a far coarser quantisation than `f64` is. Nothing in this module feeds back
//! into stored or transmitted values.

use chrono::NaiveDate;

use crate::data::Bar;

#[derive(Debug, Clone, Copy)]
pub struct ChartOptions {
    pub width: usize,
    pub height: usize,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            width: 96,
            height: 20,
        }
    }
}

/// Draws a high-low bar chart.
///
/// High-low rather than a close-only line because the range is what shows a
/// suspicious bar: a spike that never traded, or a day whose high and low are
/// identical, is invisible on a line of closes.
pub fn render(bars: &[Bar], options: ChartOptions) -> String {
    if bars.is_empty() {
        return "(no bars)".to_owned();
    }
    let width = options.width.max(1).min(bars.len().max(1));
    let height = options.height.max(1);

    // Each column covers a contiguous run of bars, so the chart stays the same
    // width whether the series is 30 bars or 3000.
    let columns: Vec<(f64, f64)> = (0..width)
        .map(|column| {
            let from = column * bars.len() / width;
            let to = ((column + 1) * bars.len() / width).max(from + 1);
            let slice = &bars[from..to.min(bars.len())];
            let low = slice
                .iter()
                .map(|bar| as_f64(&bar.low))
                .fold(f64::MAX, f64::min);
            let high = slice
                .iter()
                .map(|bar| as_f64(&bar.high))
                .fold(f64::MIN, f64::max);
            (low, high)
        })
        .collect();

    let lowest = columns.iter().map(|(low, _)| *low).fold(f64::MAX, f64::min);
    let highest = columns
        .iter()
        .map(|(_, high)| *high)
        .fold(f64::MIN, f64::max);
    // A perfectly flat series would divide by zero; give it a band to sit in.
    let span = if highest > lowest {
        highest - lowest
    } else {
        1.0
    };

    let label_width = format!("{highest:.2}")
        .len()
        .max(format!("{lowest:.2}").len());
    let mut out = String::new();
    for row in 0..height {
        // Row 0 is the top of the chart, so prices descend as `row` grows.
        let upper = highest - span * (row as f64) / height as f64;
        let lower = highest - span * ((row + 1) as f64) / height as f64;

        let label = match row {
            0 => format!("{highest:>label_width$.2}"),
            _ if row == height - 1 => format!("{lowest:>label_width$.2}"),
            _ => " ".repeat(label_width),
        };
        out.push_str(&label);
        out.push_str(" |");
        for (low, high) in &columns {
            // A cell is filled when the bar's range crosses this row's band.
            out.push(if *high >= lower && *low <= upper {
                '█'
            } else {
                ' '
            });
        }
        out.push('\n');
    }

    out.push_str(&" ".repeat(label_width));
    out.push(' ');
    out.push_str(&"-".repeat(width + 1));
    out.push('\n');
    out.push_str(&" ".repeat(label_width + 2));
    out.push_str(&axis_labels(bars[0].date, bars[bars.len() - 1].date, width));
    out.push('\n');
    out
}

fn axis_labels(first: NaiveDate, last: NaiveDate, width: usize) -> String {
    let (first, last) = (first.to_string(), last.to_string());
    if width <= first.len() + last.len() + 1 {
        return format!("{first} .. {last}");
    }
    format!(
        "{first}{}{last}",
        " ".repeat(width - first.len() - last.len())
    )
}

/// One-line facts a chart cannot show precisely.
pub fn summary(bars: &[Bar]) -> String {
    let Some((first, last)) = bars.first().zip(bars.last()) else {
        return "no bars".to_owned();
    };
    let open = as_f64(&first.close);
    let close = as_f64(&last.close);
    let change = if open > 0.0 {
        (close / open - 1.0) * 100.0
    } else {
        f64::NAN
    };

    let mut parts = vec![
        format!("{} bars", bars.len()),
        format!("{} .. {}", first.date, last.date),
        format!("close {:.2} -> {:.2} ({change:+.1}%)", open, close),
    ];

    // A gap longer than a long weekend is usually missing data rather than a
    // holiday, and it is invisible in a chart that just draws columns.
    if let Some((from, to, days)) = largest_gap(bars)
        && days > 4
    {
        parts.push(format!("largest gap {days}d ({from} .. {to})"));
    }
    parts.join(", ")
}

fn largest_gap(bars: &[Bar]) -> Option<(NaiveDate, NaiveDate, i64)> {
    bars.windows(2)
        .map(|pair| {
            (
                pair[0].date,
                pair[1].date,
                (pair[1].date - pair[0].date).num_days(),
            )
        })
        .max_by_key(|(_, _, days)| *days)
}

/// Display-only conversion; see the module docs.
fn as_f64(value: &crate::types::manual::Numeric) -> f64 {
    use rust_decimal::prelude::ToPrimitive;
    value.0.to_f64().unwrap_or(f64::NAN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::manual::Numeric;

    fn bar(date: &str, low: &str, high: &str) -> Bar {
        Bar {
            date: date.parse().unwrap(),
            open: Numeric(low.parse().unwrap()),
            high: Numeric(high.parse().unwrap()),
            low: Numeric(low.parse().unwrap()),
            close: Numeric(high.parse().unwrap()),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
        }
    }

    #[test]
    fn an_empty_series_renders_without_panicking() {
        assert_eq!(render(&[], ChartOptions::default()), "(no bars)");
        assert_eq!(summary(&[]), "no bars");
    }

    #[test]
    fn a_flat_series_does_not_divide_by_zero() {
        let bars: Vec<Bar> = (1..=5)
            .map(|day| bar(&format!("2026-08-0{day}"), "100", "100"))
            .collect();
        let chart = render(&bars, ChartOptions::default());
        assert!(chart.contains('█'));
    }

    #[test]
    fn a_single_bar_renders() {
        let chart = render(&[bar("2026-08-03", "9", "11")], ChartOptions::default());
        assert!(chart.contains("11.00"));
    }

    #[test]
    fn the_chart_is_the_requested_height() {
        let bars: Vec<Bar> = (1..=9)
            .map(|day| bar(&format!("2026-08-0{day}"), "10", "20"))
            .collect();
        let options = ChartOptions {
            width: 8,
            height: 5,
        };
        // Five rows, one axis rule, one label line.
        assert_eq!(render(&bars, options).lines().count(), 7);
    }

    #[test]
    fn summary_reports_the_span_and_the_move() {
        let bars = vec![
            bar("2026-08-03", "100", "100"),
            bar("2026-08-04", "110", "110"),
        ];
        let text = summary(&bars);
        assert!(text.contains("2 bars"), "{text}");
        assert!(text.contains("+10.0%"), "{text}");
    }

    #[test]
    fn a_long_gap_is_reported_but_a_weekend_is_not() {
        // Friday to Monday is normal and should stay quiet.
        let weekend = vec![
            bar("2026-08-07", "100", "100"),
            bar("2026-08-10", "100", "100"),
        ];
        assert!(!summary(&weekend).contains("gap"));

        // Three weeks missing is a data problem worth naming.
        let missing = vec![
            bar("2026-08-03", "100", "100"),
            bar("2026-08-24", "100", "100"),
        ];
        assert!(
            summary(&missing).contains("largest gap 21d"),
            "{}",
            summary(&missing)
        );
    }
}
