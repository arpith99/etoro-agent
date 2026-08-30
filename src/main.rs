use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use chrono::{Datelike, NaiveDate, Utc};
use etoro_agent::analysis::gaps;
use etoro_agent::chart::{ChartOptions, contains_split, render, render_html, summary};
use etoro_agent::client::EtoroClient;
use etoro_agent::data::{
    BarSource, DataError, DateRange, PriceBasis, SeriesChoice,
    store::{BarStore, FileStore, Interval, SeriesKey},
    tiingo::Tiingo,
};
use serde::Serialize;

const USAGE: &str = "\
usage:
  etoro-agent                                  account summary
  etoro-agent prices  SYM[,SYM...]             live bid/ask from eToro
  etoro-agent fetch-bars SYM[,SYM...] [FROM] [TO]
                                               daily bars from Tiingo into the store
  etoro-agent gaps    SYM[,SYM...] [SOURCE]   split returns into overnight and
                                               intraday parts; one symbol gives
                                               detail, several give a table
  etoro-agent chart   SYM [SOURCE] [--html [FILE]]
                                               draw a stored series; --html writes
                                               an interactive candlestick page

dates are YYYY-MM-DD; FROM defaults to five years ago and TO to today.
SOURCE defaults to tiingo.
";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    match Command::parse(std::env::args().skip(1).collect())? {
        Command::Summary { symbols } => account_summary(symbols).await,
        Command::FetchBars { symbols, range } => fetch_bars(symbols, range).await,
        Command::Gaps { symbols, source } => gaps_report(&symbols, &source),
        Command::Chart {
            symbol,
            source,
            html,
        } => chart(&symbol, &source, html.as_deref()),
    }
}

/// Splits stored series' returns into overnight and intraday parts.
///
/// One symbol prints the full report; several print a row each, which is what
/// answers whether an effect seen in one name is general or idiosyncratic.
///
/// Uses the total-return series when the source has one. On as-traded prices
/// an ex-dividend date reads as an overnight loss no holder suffered, and a
/// split reads as an overnight collapse -- both would land straight in the
/// "largest overnight moves" list and crowd out the real ones.
fn gaps_report(symbols: &[String], source: &str) -> Result<()> {
    let store = FileStore::new(store_root());
    let mut rows = Vec::new();

    for symbol in symbols {
        let key = SeriesKey::new(source, symbol, Interval::Daily)?;
        // One missing ticker should not abandon the comparison.
        let Some(series) = store.load(&key)? else {
            println!("{symbol}: not in the store; run: etoro-agent fetch-bars {symbol}");
            continue;
        };
        let adjusted = series
            .bars
            .iter()
            .any(|bar| bar.total_return_close.is_some());
        let choice = if adjusted {
            SeriesChoice::TotalReturn
        } else {
            SeriesChoice::Reported
        };
        match gaps(&series.bars, choice, GAP_SAMPLE) {
            Some(stats) => rows.push((symbol.clone(), adjusted, stats)),
            None => println!("{symbol}: too few bars to measure gaps"),
        }
    }

    let Some((first_symbol, first_adjusted, first_stats)) = rows.first() else {
        bail!("no stored series to report on");
    };

    if rows.len() == 1 {
        println!(
            "{} {} ({} prices from {source}, {} sessions)",
            first_symbol.to_ascii_uppercase(),
            Interval::Daily.as_str(),
            if *first_adjusted {
                "TotalReturn"
            } else {
                "Reported"
            },
            first_stats.sessions
        );
        if !first_adjusted {
            println!(
                "! no adjusted closes in this series, so dividends and splits will \
                 show up as overnight moves"
            );
        }
        println!();
        print!("{}", first_stats.report());
        return Ok(());
    }

    // Annualised throughout, because the windows differ per symbol -- a
    // compounded figure over 5 years and one over 3.8 are not comparable.
    println!(
        "{:<8}{:>9}{:>11}{:>7}{:>10}{:>7}{:>8}{:>9}",
        "symbol", "sessions", "overnight", "vol", "intraday", "vol", "corr", "total"
    );
    let mut any_unadjusted = false;
    for (symbol, adjusted, stats) in &rows {
        any_unadjusted |= !adjusted;
        println!(
            "{:<8}{:>9}{:>11}{:>7}{:>10}{:>7}{:>8}{:>9}",
            format!(
                "{}{}",
                symbol.to_ascii_uppercase(),
                if *adjusted { "" } else { "*" }
            ),
            stats.sessions,
            format!("{:+.2}%", stats.overnight.annualised * 100.0),
            format!("{:.1}%", stats.overnight.volatility * 100.0),
            format!("{:+.2}%", stats.intraday.annualised * 100.0),
            format!("{:.1}%", stats.intraday.volatility * 100.0),
            if stats.correlation.is_finite() {
                format!("{:+.2}", stats.correlation)
            } else {
                "-".to_owned()
            },
            format!("{:+.2}%", stats.total.annualised * 100.0),
        );
    }
    println!("\nreturns and volatility are annualised; corr is overnight against intraday.");
    if any_unadjusted {
        println!("* no adjusted closes, so dividends and splits appear as overnight moves.");
    }
    Ok(())
}

/// Draws a stored series. Reads the local store only -- no network, no
/// credentials, so it stays usable when a vendor is down.
fn chart(symbol: &str, source: &str, html: Option<&Path>) -> Result<()> {
    let store = FileStore::new(store_root());
    let key = SeriesKey::new(source, symbol, Interval::Daily)?;
    let Some(series) = store.load(&key)? else {
        bail!("no stored series for {symbol} from {source}; run: etoro-agent fetch-bars {symbol}");
    };

    // The basis is printed, not assumed. Two series can look identical and
    // mean different things, which is the whole reason it is stored.
    println!(
        "{} {} ({:?} prices from {})",
        symbol.to_ascii_uppercase(),
        Interval::Daily.as_str(),
        series.basis,
        source
    );
    let subtitle = summary(&series.bars, ChartOptions::default().series);
    println!("{subtitle}");

    // Without this the reader has to know what AsTraded implies. TSLA's 2022
    // split turns a real +43% into a displayed -52%, and nothing about the
    // chart says so.
    if series.basis == PriceBasis::AsTraded && contains_split(&series.bars) {
        println!(
            "! this series contains a split and holds as-traded prices, so the \
             change above is not a return"
        );
    }

    match html {
        Some(path) => {
            let title = format!(
                "{} {}",
                symbol.to_ascii_uppercase(),
                Interval::Daily.as_str()
            );
            let subtitle = format!("{:?} prices from {source} - {subtitle}", series.basis);
            std::fs::write(path, render_html(&series.bars, &title, &subtitle))
                .with_context(|| format!("writing {}", path.display()))?;
            println!("Wrote {}", path.display());
        }
        None => {
            println!();
            print!("{}", render(&series.bars, ChartOptions::default()));
        }
    }
    Ok(())
}

/// Fetches daily bars into the local store.
///
/// Deliberately requires no eToro credentials: this reaches Tiingo only, and
/// making it depend on keys it does not use would be a reason not to run it.
async fn fetch_bars(symbols: Vec<String>, range: DateRange) -> Result<()> {
    let token = std::env::var("TIINGO_API_KEY")
        .context("TIINGO_API_KEY is needed to fetch bars; see scripts/README.md")?;
    let source = Tiingo::new(&token)?;
    let store = FileStore::new(store_root());

    println!(
        "Fetching {} .. {} from {} into {}",
        range.start,
        range.end,
        source.name(),
        store_root().display()
    );

    for symbol in &symbols {
        let bars = match source.daily_bars(symbol, range).await {
            Ok(bars) => bars,
            // One bad ticker should not abandon the rest of the batch.
            Err(DataError::UnknownSymbol { .. }) => {
                println!("  {symbol}: not a ticker {} knows", source.name());
                continue;
            }
            Err(error) => return Err(error.into()),
        };

        let key = SeriesKey::new(source.name(), symbol, Interval::Daily)?;
        // The basis comes from the source rather than being assumed here: the
        // store refuses to mix conventions, and this is where the two meet.
        let outcome = store.merge(&key, source.basis(), &bars)?;
        println!(
            "  {symbol}: {} fetched, {} added, {} updated, {} unchanged, {} stored",
            bars.len(),
            outcome.added,
            outcome.updated,
            outcome.unchanged,
            outcome.total,
        );
    }
    Ok(())
}

/// Where series are written. Market data, not account data, so it carries no
/// special permissions -- but it is gitignored, being derived and large.
fn store_root() -> PathBuf {
    std::env::var("ETORO_AGENT_STORE")
        .unwrap_or_else(|_| "market-data".to_owned())
        .into()
}

async fn account_summary(symbols: Vec<String>) -> Result<()> {
    let api_key = std::env::var("ETORO_API_KEY")?;
    let user_key = std::env::var("ETORO_USER_KEY")?;
    let dump_responses = env_flag("ETORO_DUMP_RESPONSES");

    let client = EtoroClient::new(&api_key, &user_key)?;

    let watchlists = client.watchlists().await?;
    println!("Fetched {} watchlist(s).", watchlists.watchlists.len());
    if dump_responses {
        dump_private("watchlists_response.json", &watchlists)?;
    }

    let portfolio = client.portfolio().await?;
    let position_count = portfolio
        .client_portfolio
        .as_ref()
        .map_or(0, |portfolio| portfolio.positions.len());
    println!("Fetched a portfolio with {position_count} open position(s).");
    if dump_responses {
        dump_private("portfolio_response.json", &portfolio)?;
    }

    let me = client.me().await?;
    println!("Fetched user details for user: {}.", me.username);
    if dump_responses {
        dump_private("me_response.json", &me)?;
    }

    // Resolution is one call per symbol -- the search filter takes a single
    // value -- but pricing is a single call for the whole set, which is the
    // batching that matters: `rates` accepts up to 100 IDs at once.
    let mut resolved: Vec<(String, i64)> = Vec::new();
    for symbol in &symbols {
        match client.resolve_symbol(symbol).await? {
            Some(instrument_id) => resolved.push((symbol.clone(), instrument_id)),
            None => println!("{symbol}: no instrument matched that symbol."),
        }
    }

    if !resolved.is_empty() {
        let instrument_ids: Vec<i64> = resolved.iter().map(|(_, id)| *id).collect();
        let rates = client.rates(&instrument_ids).await?;
        for (symbol, instrument_id) in &resolved {
            // Join on the ID rather than on position: the API returns rates for
            // the instruments it recognises, in no guaranteed order, and omits
            // the rest.
            match rates
                .rates
                .iter()
                .find(|rate| rate.instrument_id == Some(*instrument_id))
            {
                // Every field is optional in the schema, so nothing here is
                // unwrapped -- a rate with no bid is a real thing the API sends.
                Some(rate) => println!(
                    "{symbol} ({instrument_id}): bid {} ask {}",
                    display(rate.bid.as_ref()),
                    display(rate.ask.as_ref()),
                ),
                // Resolved but unpriced: the instrument exists and the market
                // data does not, which is not the same as an unknown symbol.
                None => println!("{symbol} ({instrument_id}): no rate returned."),
            }
        }
        if dump_responses {
            dump_private("rates_response.json", &rates)?;
        }
    }

    Ok(())
}

/// Symbol used when none is given on the command line.
const DEFAULT_SYMBOL: &str = "AAPL";

/// How far back `fetch-bars` reaches when no start date is given.
const DEFAULT_HISTORY_YEARS: i32 = 5;

/// Bar source assumed when `chart` is not told one.
const DEFAULT_SOURCE: &str = "tiingo";

/// How many outlier sessions `gaps` lists.
const GAP_SAMPLE: usize = 8;

/// What the binary was asked to do.
#[derive(Debug, PartialEq)]
enum Command {
    Summary {
        symbols: Vec<String>,
    },
    FetchBars {
        symbols: Vec<String>,
        range: DateRange,
    },
    Gaps {
        symbols: Vec<String>,
        source: String,
    },
    Chart {
        symbol: String,
        source: String,
        /// `None` renders in the terminal; `Some(path)` writes an HTML page.
        html: Option<PathBuf>,
    },
}

impl Command {
    /// Parsed from owned arguments rather than read from the environment, so
    /// the dispatch is testable without a process.
    fn parse(args: Vec<String>) -> Result<Self> {
        let today = Utc::now().date_naive();
        match args.split_first() {
            None => Ok(Self::Summary {
                symbols: parse_symbols(DEFAULT_SYMBOL),
            }),
            Some((verb, rest)) if verb == "prices" => Ok(Self::Summary {
                symbols: parse_symbols(rest.first().map_or(DEFAULT_SYMBOL, String::as_str)),
            }),
            Some((verb, rest)) if verb == "fetch-bars" => {
                let Some(symbols) = rest.first() else {
                    bail!("fetch-bars needs at least one symbol\n\n{USAGE}");
                };
                let start = match rest.get(1) {
                    Some(text) => parse_date(text)?,
                    None => today
                        .with_year(today.year() - DEFAULT_HISTORY_YEARS)
                        // 29 February has no counterpart in a common year.
                        .unwrap_or(today),
                };
                let end = match rest.get(2) {
                    Some(text) => parse_date(text)?,
                    None => today,
                };
                Ok(Self::FetchBars {
                    symbols: parse_symbols(symbols),
                    range: DateRange::new(start, end).map_err(|detail| anyhow!(detail))?,
                })
            }
            Some((verb, rest)) if verb == "gaps" => {
                let Some(symbols) = rest.first() else {
                    bail!("gaps needs at least one symbol\n\n{USAGE}");
                };
                Ok(Self::Gaps {
                    symbols: parse_symbols(symbols),
                    source: rest
                        .get(1)
                        .cloned()
                        .unwrap_or_else(|| DEFAULT_SOURCE.to_owned()),
                })
            }
            Some((verb, rest)) if verb == "chart" => {
                let (mut positional, mut html, mut html_path) = (Vec::new(), false, None);
                let mut args = rest.iter();
                while let Some(arg) = args.next() {
                    match arg.as_str() {
                        "--html" => {
                            html = true;
                            // An optional path follows, but only if the next
                            // token is not itself a flag.
                            html_path = args
                                .clone()
                                .next()
                                .filter(|next| !next.starts_with("--"))
                                .map(|next| {
                                    args.next();
                                    PathBuf::from(next)
                                });
                        }
                        other if other.starts_with("--") => {
                            bail!("unknown option {other:?}\n\n{USAGE}")
                        }
                        other => positional.push(other.to_owned()),
                    }
                }
                let Some(symbol) = positional.first() else {
                    bail!("chart needs a symbol\n\n{USAGE}");
                };
                Ok(Self::Chart {
                    symbol: symbol.clone(),
                    source: positional
                        .get(1)
                        .cloned()
                        .unwrap_or_else(|| DEFAULT_SOURCE.to_owned()),
                    html: html.then(|| {
                        html_path.unwrap_or_else(|| {
                            PathBuf::from(format!("{}.html", symbol.to_ascii_lowercase()))
                        })
                    }),
                })
            }
            // A bare symbol list used to mean "price these". Rejecting it is
            // better than guessing, now that a verb could also be a ticker.
            Some((other, _)) => bail!("unknown command {other:?}\n\n{USAGE}"),
        }
    }
}

fn parse_date(text: &str) -> Result<NaiveDate> {
    text.parse()
        .with_context(|| format!("{text:?} is not a YYYY-MM-DD date"))
}

fn parse_symbols(argument: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    argument
        .split(',')
        .map(str::trim)
        .filter(|symbol| !symbol.is_empty())
        .filter(|symbol| seen.insert(symbol.to_ascii_uppercase()))
        .map(str::to_owned)
        .collect()
}

/// Renders an optional field without unwrapping it.
fn display<T: std::fmt::Display>(value: Option<&T>) -> String {
    value.map_or_else(|| "-".to_owned(), T::to_string)
}

fn env_flag(name: &str) -> bool {
    std::env::var(name).is_ok_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes"
        )
    })
}

fn dump_private<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(value)?;
    write_private(path, json.as_bytes())?;
    println!("Wrote {} bytes to {}.", json.len(), path.display());
    Ok(())
}

/// Writes `contents` to a fresh file that only the current user can read.
///
/// Whatever currently sits at `path` (a stale dump, or a symlink somebody
/// planted there) is unlinked first, then the file is created with
/// `create_new`, i.e. `O_CREAT | O_EXCL`. `O_EXCL` never follows a symlink and
/// fails if the path reappears between the unlink and the create, so we never
/// truncate or write through a link to some other file. The mode is applied
/// at creation, before any bytes are written.
#[cfg(unix)]
fn write_private(path: &Path, contents: &[u8]) -> Result<()> {
    use std::fs::{OpenOptions, Permissions};
    use std::io::{ErrorKind, Write};
    use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

    match std::fs::remove_file(path) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    // `mode` is filtered through the umask; pin the exact permissions.
    file.set_permissions(Permissions::from_mode(0o600))?;
    file.write_all(contents)?;
    Ok(())
}

#[cfg(not(unix))]
fn write_private(path: &Path, contents: &[u8]) -> Result<()> {
    std::fs::write(path, contents)?;
    Ok(())
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;
    use std::path::PathBuf;

    use super::*;

    /// A unique temp path that is removed on drop, so a failing assertion
    /// does not leave files behind.
    struct TempPath(PathBuf);

    impl TempPath {
        fn new(suffix: &str) -> Self {
            Self(std::env::temp_dir().join(format!(
                "etoro-agent-test-{}-{suffix}",
                uuid::Uuid::new_v4()
            )))
        }
    }

    impl Drop for TempPath {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn mode_of(path: &Path) -> u32 {
        std::fs::metadata(path).unwrap().permissions().mode() & 0o777
    }

    #[test]
    fn no_arguments_means_an_account_summary() {
        let command = Command::parse(vec![]).unwrap();
        assert_eq!(
            command,
            Command::Summary {
                symbols: vec!["AAPL".to_owned()]
            }
        );
    }

    #[test]
    fn prices_takes_a_symbol_list() {
        let command = Command::parse(vec!["prices".to_owned(), "AAPL,MSFT".to_owned()]).unwrap();
        assert_eq!(
            command,
            Command::Summary {
                symbols: vec!["AAPL".to_owned(), "MSFT".to_owned()]
            }
        );
    }

    #[test]
    fn fetch_bars_defaults_to_five_years_ending_today() {
        let Command::FetchBars { range, symbols } =
            Command::parse(vec!["fetch-bars".to_owned(), "AAPL".to_owned()]).unwrap()
        else {
            panic!("expected FetchBars");
        };
        assert_eq!(symbols, vec!["AAPL".to_owned()]);
        let today = Utc::now().date_naive();
        assert_eq!(range.end, today);
        assert_eq!(range.start.year(), today.year() - DEFAULT_HISTORY_YEARS);
    }

    #[test]
    fn fetch_bars_accepts_an_explicit_range() {
        let Command::FetchBars { range, .. } = Command::parse(vec![
            "fetch-bars".to_owned(),
            "AAPL".to_owned(),
            "2020-01-01".to_owned(),
            "2020-12-31".to_owned(),
        ])
        .unwrap() else {
            panic!("expected FetchBars");
        };
        assert_eq!(range.start.to_string(), "2020-01-01");
        assert_eq!(range.end.to_string(), "2020-12-31");
    }

    #[test]
    fn gaps_takes_a_symbol_list_and_an_optional_source() {
        assert_eq!(
            Command::parse(vec!["gaps".to_owned(), "AAPL".to_owned()]).unwrap(),
            Command::Gaps {
                symbols: vec!["AAPL".to_owned()],
                source: "tiingo".to_owned()
            }
        );
        assert_eq!(
            Command::parse(vec![
                "gaps".to_owned(),
                "AAPL,MBLY,tsla".to_owned(),
                "etoro".to_owned()
            ])
            .unwrap(),
            Command::Gaps {
                // Same splitting, trimming and dedup as everywhere else.
                symbols: vec!["AAPL".to_owned(), "MBLY".to_owned(), "tsla".to_owned()],
                source: "etoro".to_owned()
            }
        );
        assert!(Command::parse(vec!["gaps".to_owned()]).is_err());
    }

    #[test]
    fn chart_defaults_to_the_tiingo_store_but_accepts_another() {
        assert_eq!(
            Command::parse(vec!["chart".to_owned(), "AAPL".to_owned()]).unwrap(),
            Command::Chart {
                symbol: "AAPL".to_owned(),
                source: "tiingo".to_owned(),
                html: None
            }
        );
        assert_eq!(
            Command::parse(vec![
                "chart".to_owned(),
                "AAPL".to_owned(),
                "etoro".to_owned()
            ])
            .unwrap(),
            Command::Chart {
                symbol: "AAPL".to_owned(),
                source: "etoro".to_owned(),
                html: None
            }
        );
        assert!(Command::parse(vec!["chart".to_owned()]).is_err());
    }

    #[test]
    fn html_output_defaults_its_filename_but_takes_one() {
        let bare = Command::parse(vec![
            "chart".to_owned(),
            "AAPL".to_owned(),
            "--html".to_owned(),
        ])
        .unwrap();
        assert_eq!(
            bare,
            Command::Chart {
                symbol: "AAPL".to_owned(),
                source: "tiingo".to_owned(),
                html: Some(PathBuf::from("aapl.html")),
            }
        );

        let named = Command::parse(vec![
            "chart".to_owned(),
            "AAPL".to_owned(),
            "--html".to_owned(),
            "/tmp/x.html".to_owned(),
        ])
        .unwrap();
        let Command::Chart { html, source, .. } = named else {
            panic!("expected Chart");
        };
        // The path must not be mistaken for the source argument.
        assert_eq!(html, Some(PathBuf::from("/tmp/x.html")));
        assert_eq!(source, "tiingo");

        assert!(
            Command::parse(vec![
                "chart".to_owned(),
                "AAPL".to_owned(),
                "--nope".to_owned()
            ])
            .is_err()
        );
    }

    #[test]
    fn a_backwards_range_is_refused_rather_than_returning_nothing() {
        assert!(
            Command::parse(vec![
                "fetch-bars".to_owned(),
                "AAPL".to_owned(),
                "2026-12-31".to_owned(),
                "2020-01-01".to_owned(),
            ])
            .is_err()
        );
    }

    #[test]
    fn unusable_input_is_rejected_with_usage_rather_than_guessed_at() {
        // A bare symbol list used to mean "price these". Now that a verb could
        // itself look like a ticker, guessing would be worse than refusing.
        assert!(Command::parse(vec!["AAPL".to_owned()]).is_err());
        assert!(Command::parse(vec!["fetch-bars".to_owned()]).is_err());
        assert!(
            Command::parse(vec![
                "fetch-bars".to_owned(),
                "AAPL".to_owned(),
                "last-tuesday".to_owned()
            ])
            .is_err()
        );
    }

    #[test]
    fn symbols_are_split_trimmed_and_deduplicated() {
        assert_eq!(parse_symbols("AAPL,TSLA,MSFT"), ["AAPL", "TSLA", "MSFT"]);
        assert_eq!(parse_symbols("  AAPL ,  TSLA  "), ["AAPL", "TSLA"]);
        // Empty segments come from a trailing or doubled comma, which is a
        // typo rather than a request for an empty symbol.
        assert_eq!(parse_symbols("AAPL,,TSLA,"), ["AAPL", "TSLA"]);
    }

    #[test]
    fn duplicate_symbols_cost_only_one_lookup() {
        // Matching is case-insensitive, so these name one instrument and must
        // not produce two resolution calls.
        assert_eq!(parse_symbols("AAPL,aapl,AaPl"), ["AAPL"]);
        // The first spelling is the one echoed back.
        assert_eq!(parse_symbols("aapl,AAPL"), ["aapl"]);
    }

    #[test]
    fn dump_file_is_private() {
        let path = TempPath::new("dump.json");
        dump_private(&path.0, &serde_json::json!({"sanitized": true})).unwrap();
        assert_eq!(mode_of(&path.0), 0o600);
    }

    #[test]
    fn dump_replaces_existing_world_readable_file() {
        let path = TempPath::new("existing.json");
        std::fs::write(&path.0, b"stale").unwrap();
        std::fs::set_permissions(&path.0, std::fs::Permissions::from_mode(0o644)).unwrap();

        dump_private(&path.0, &serde_json::json!({"fresh": true})).unwrap();

        assert_eq!(mode_of(&path.0), 0o600);
        assert_eq!(
            std::fs::read_to_string(&path.0).unwrap(),
            "{\n  \"fresh\": true\n}"
        );
    }

    #[test]
    fn dump_does_not_write_through_a_symlink() {
        let target = TempPath::new("target.json");
        let link = TempPath::new("link.json");
        std::fs::write(&target.0, b"untouched").unwrap();
        std::os::unix::fs::symlink(&target.0, &link.0).unwrap();

        dump_private(&link.0, &serde_json::json!({"fresh": true})).unwrap();

        // The link was replaced by a private regular file; the target kept its bytes.
        assert!(!std::fs::symlink_metadata(&link.0).unwrap().is_symlink());
        assert_eq!(mode_of(&link.0), 0o600);
        assert_eq!(std::fs::read_to_string(&target.0).unwrap(), "untouched");
    }
}
