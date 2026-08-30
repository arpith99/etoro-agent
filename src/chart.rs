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

/// Which price series to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeriesChoice {
    /// Whatever the source reported, on its own basis.
    Reported,
    /// The dividend- and split-adjusted series, where the source has one.
    TotalReturn,
}

#[derive(Debug, Clone, Copy)]
pub struct ChartOptions {
    pub width: usize,
    pub height: usize,
    pub series: SeriesChoice,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            width: 96,
            height: 20,
            series: SeriesChoice::Reported,
        }
    }
}

/// A bar's open, high, low and close on the chosen series.
///
/// Only the adjusted *close* is stored, so adjusted open/high/low are derived
/// from the ratio between the two closes. That ratio is the session's
/// cumulative adjustment factor and applies to every price in the bar, so
/// scaling by it is exact rather than an approximation.
fn prices(bar: &Bar, series: SeriesChoice) -> [f64; 4] {
    let raw = [
        as_f64(&bar.open),
        as_f64(&bar.high),
        as_f64(&bar.low),
        as_f64(&bar.close),
    ];
    match series {
        SeriesChoice::Reported => raw,
        SeriesChoice::TotalReturn => match bar.total_return_close.as_ref() {
            Some(adjusted) if raw[3] > 0.0 => {
                let factor = as_f64(adjusted) / raw[3];
                raw.map(|price| price * factor)
            }
            // No adjusted close: report the raw prices rather than fabricate
            // an adjustment. Callers are warned separately.
            _ => raw,
        },
    }
}

/// True when the series records a split.
///
/// A split makes an as-traded chart actively misleading: the discontinuity
/// reads as a crash when what changed was the share count. TSLA's 3:1 in 2022
/// turns a real +43% into a displayed -52%.
pub fn contains_split(bars: &[Bar]) -> bool {
    bars.iter().any(|bar| {
        bar.split_factor
            .as_ref()
            .is_some_and(|factor| as_f64(factor) != 1.0)
    })
}

/// True when the series carries adjusted closes, so [`SeriesChoice::TotalReturn`]
/// shows something different rather than silently falling back.
///
/// `any` rather than `all`: a vendor omitting one session's adjusted close is
/// not a reason to hide the whole comparison, and [`prices`] already falls back
/// per bar for the ones that lack it.
pub fn has_total_return(bars: &[Bar]) -> bool {
    bars.iter().any(|bar| bar.total_return_close.is_some())
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
                .map(|bar| prices(bar, options.series)[2])
                .fold(f64::MAX, f64::min);
            let high = slice
                .iter()
                .map(|bar| prices(bar, options.series)[1])
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
pub fn summary(bars: &[Bar], series: SeriesChoice) -> String {
    let Some((first, last)) = bars.first().zip(bars.last()) else {
        return "no bars".to_owned();
    };
    let open = prices(first, series)[3];
    let close = prices(last, series)[3];
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

/// A self-contained interactive candlestick page.
///
/// Zoom, pan, crosshair with an OHLC legend, a volume pane, moving-average
/// overlays, range presets and a log-scale toggle. The charting library is
/// embedded rather than linked from a CDN, so the file works offline, renders
/// identically later, and does not announce to a third party every time
/// somebody looks at their own portfolio.
///
/// Both series are written into the page when the source carries adjusted
/// closes, and the page can switch between them. That is the point rather than
/// a flourish: an as-traded chart of a stock that split shows a crash that did
/// not happen, and being able to flip between the two makes the difference
/// visible instead of a footnote.
///
/// The page's CSS and JavaScript live in `assets/` rather than inside this
/// format string. Both are brace-heavy, and doubling every one of them to
/// satisfy `format!` made them unreadable and uncheckable.
pub fn render_html(bars: &[Bar], title: &str, subtitle: &str) -> String {
    let points = |series: SeriesChoice| -> Vec<serde_json::Value> {
        bars.iter()
            .map(|bar| {
                let [open, high, low, close] = prices(bar, series);
                serde_json::json!({
                    "time": bar.date.to_string(),
                    "open": open, "high": high, "low": low, "close": close,
                })
            })
            .collect()
    };

    // Volume is emitted twice for the same reason prices are. A split changes
    // the share count, so on an as-traded series the volume bars step at the
    // split date just as the price does -- and toggling the price without the
    // volume would leave the pane below quietly wrong.
    let volume_points = |series: SeriesChoice| -> Vec<serde_json::Value> {
        bars.iter()
            .filter_map(|bar| {
                let value = match series {
                    SeriesChoice::Reported => bar.volume.as_ref()?,
                    // Fall back rather than fabricate, as with prices.
                    SeriesChoice::TotalReturn => {
                        bar.split_adjusted_volume.as_ref().or(bar.volume.as_ref())?
                    }
                };
                // Direction is unaffected by adjustment: scaling open and close
                // by the same factor cannot flip which is larger.
                let rising = as_f64(&bar.close) >= as_f64(&bar.open);
                Some(serde_json::json!({
                    "time": bar.date.to_string(),
                    "value": as_f64(value),
                    // Muted, so volume reads as context rather than competing
                    // with the price bars above it.
                    "color": if rising { "rgba(38,166,154,0.4)" } else { "rgba(239,83,80,0.4)" },
                }))
            })
            .collect()
    };
    let has_adjusted_volume = bars.iter().any(|bar| bar.split_adjusted_volume.is_some());

    let data = serde_json::json!({
        "reported": points(SeriesChoice::Reported),
        "adjusted": if has_total_return(bars) {
            serde_json::Value::from(points(SeriesChoice::TotalReturn))
        } else {
            serde_json::Value::Null
        },
        "volume": volume_points(SeriesChoice::Reported),
        "volumeAdjusted": if has_adjusted_volume {
            serde_json::Value::from(volume_points(SeriesChoice::TotalReturn))
        } else {
            serde_json::Value::Null
        },
    });
    // `</script>` inside a script block would end it early. The values here are
    // numbers and dates, but escaping the sequence costs nothing and removes
    // the need to keep believing that.
    let data = serde_json::to_string(&data)
        .unwrap_or_else(|_| "{}".to_owned())
        .replace("</", r"<\/");

    let warning = if contains_split(bars) {
        format!(
            "<div class=\"warn\">{}</div>",
            html_escape(
                "This series contains a split. On as-traded prices the split reads as a \
                 crash that never happened - switch to total return to see the real path."
            )
        )
    } else {
        String::new()
    };

    format!(
        "<!doctype html>\n\
         <meta charset=\"utf-8\">\n\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n\
         <title>{title}</title>\n\
         <style>{css}</style>\n\
         <h1>{title}</h1>\n\
         <div class=\"sub\">{subtitle}</div>\n\
         {warning}\
         <div class=\"bar\" id=\"controls\"></div>\n\
         <div id=\"legend\"></div>\n\
         <div id=\"chart\"></div>\n\
         <script>window.CHART_DATA = {data};</script>\n\
         <script>{library}</script>\n\
         <script>{app}</script>\n",
        title = html_escape(title),
        subtitle = html_escape(subtitle),
        warning = warning,
        css = include_str!("../assets/chart.css"),
        data = data,
        library = include_str!("../assets/lightweight-charts.standalone.production.js"),
        app = include_str!("../assets/chart.js"),
    )
}

/// Escapes text interpolated into the page.
///
/// The values here are symbols and dates rather than anything hostile, but a
/// symbol arrives from the command line and the cost of being careful is four
/// replacements.
fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
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
            split_adjusted_volume: None,
        }
    }

    fn bar_with(date: &str, close: &str, adjusted: Option<&str>, split: Option<&str>) -> Bar {
        let mut b = bar(date, close, close);
        b.total_return_close = adjusted.map(|v| Numeric(v.parse().unwrap()));
        b.split_factor = split.map(|v| Numeric(v.parse().unwrap()));
        b
    }

    #[test]
    fn total_return_scales_the_whole_bar_by_the_close_ratio() {
        // Only the adjusted close is stored; open/high/low follow from the
        // ratio, which is the session's cumulative adjustment factor.
        let mut b = bar("2026-08-03", "50", "50");
        b.open = Numeric("40".parse().unwrap());
        b.high = Numeric("60".parse().unwrap());
        b.low = Numeric("30".parse().unwrap());
        b.close = Numeric("50".parse().unwrap());
        b.total_return_close = Some(Numeric("25".parse().unwrap()));

        let [open, high, low, close] = prices(&b, SeriesChoice::TotalReturn);
        assert_eq!((open, high, low, close), (20.0, 30.0, 15.0, 25.0));
    }

    #[test]
    fn a_bar_without_an_adjusted_close_falls_back_instead_of_inventing_one() {
        let b = bar_with("2026-08-03", "50", None, None);
        assert_eq!(
            prices(&b, SeriesChoice::TotalReturn),
            prices(&b, SeriesChoice::Reported)
        );
    }

    #[test]
    fn a_split_is_detected_and_a_factor_of_one_is_not() {
        assert!(contains_split(&[bar_with(
            "2026-08-03",
            "50",
            None,
            Some("3")
        )]));
        assert!(!contains_split(&[bar_with(
            "2026-08-03",
            "50",
            None,
            Some("1")
        )]));
        assert!(!contains_split(&[bar_with("2026-08-03", "50", None, None)]));
    }

    #[test]
    fn one_missing_adjusted_close_does_not_disable_the_comparison() {
        // `all` here would let a single vendor gap hide the toggle entirely.
        let bars = [
            bar_with("2026-08-03", "50", Some("25"), None),
            bar_with("2026-08-04", "51", None, None),
        ];
        assert!(has_total_return(&bars));
        assert!(!has_total_return(&[bar_with(
            "2026-08-03",
            "50",
            None,
            None
        )]));
    }

    #[test]
    fn the_page_embeds_its_library_and_both_series() {
        let bars = [
            bar_with("2026-08-03", "400", Some("100"), None),
            bar_with("2026-08-04", "100", Some("100"), Some("4")),
        ];
        let html = render_html(&bars, "AAPL 1d", "test");

        // Self-contained: nothing is loaded from elsewhere. Asserting on the
        // absence of URLs would be wrong -- the vendored library's licence
        // header contains several, and they are text rather than requests.
        assert!(
            !html.contains("<script src"),
            "page must not fetch a script"
        );
        assert!(!html.contains("<link "), "page must not fetch a stylesheet");
        assert!(html.contains("LightweightCharts"));
        // Sizing: the bug that made the chart invisible.
        assert!(html.contains("autoSize: true"));
        // Both series, and the warning the split earns.
        assert!(html.contains("\"reported\""));
        assert!(html.contains("\"adjusted\""));
        assert!(html.contains("contains a split"));
        // Exactly three script blocks: data, library, page logic.
        assert_eq!(html.matches("</script>").count(), 3);
    }

    #[test]
    fn adjusted_volume_is_emitted_when_the_source_has_it() {
        let mut split = bar_with("2026-08-04", "100", Some("100"), Some("4"));
        split.volume = Some(Numeric("1000".parse().unwrap()));
        split.split_adjusted_volume = Some(Numeric("4000".parse().unwrap()));
        let mut before = bar_with("2026-08-03", "400", Some("100"), None);
        before.volume = Some(Numeric("250".parse().unwrap()));
        before.split_adjusted_volume = Some(Numeric("1000".parse().unwrap()));

        let html = render_html(&[before, split], "AAPL 1d", "test");
        assert!(html.contains("\"volumeAdjusted\""));
        // The adjusted figure must actually be present, not just the key.
        assert!(html.contains("1000.0"), "adjusted volume missing");
    }

    #[test]
    fn no_adjusted_volume_means_no_second_volume_series() {
        let mut b = bar_with("2026-08-03", "400", Some("100"), None);
        b.volume = Some(Numeric("250".parse().unwrap()));
        let html = render_html(&[b], "AAPL 1d", "test");
        assert!(html.contains("\"volumeAdjusted\":null"));
    }

    #[test]
    fn a_series_without_adjusted_closes_offers_no_toggle() {
        let bars = [bar_with("2026-08-03", "400", None, None)];
        let html = render_html(&bars, "AAPL 1d", "test");
        assert!(html.contains("\"adjusted\":null"));
        assert!(!html.contains("contains a split"));
    }

    #[test]
    fn interpolated_text_is_escaped() {
        let html = render_html(&[bar("2026-08-03", "1", "1")], "<script>x</script>", "s");
        assert!(html.contains("&lt;script&gt;"));
        assert_eq!(html.matches("</script>").count(), 3);
    }

    #[test]
    fn an_empty_series_renders_without_panicking() {
        assert_eq!(render(&[], ChartOptions::default()), "(no bars)");
        assert_eq!(summary(&[], SeriesChoice::Reported), "no bars");
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
            series: SeriesChoice::Reported,
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
        let text = summary(&bars, SeriesChoice::Reported);
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
        assert!(!summary(&weekend, SeriesChoice::Reported).contains("gap"));

        // Three weeks missing is a data problem worth naming.
        let missing = vec![
            bar("2026-08-03", "100", "100"),
            bar("2026-08-24", "100", "100"),
        ];
        let text = summary(&missing, SeriesChoice::Reported);
        assert!(text.contains("largest gap 21d"), "{text}");
    }
}
