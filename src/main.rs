use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow, bail};
use chrono::{Datelike, NaiveDate, Utc};
use etoro_agent::analysis::gaps;
use etoro_agent::backtest::{Backtest, CostModel, FillPrice};
use etoro_agent::chart::{ChartOptions, contains_split, render, render_html, summary};
use etoro_agent::client::{Environment, EtoroClient};
use etoro_agent::data::{
    BarSource, DataError, DateRange, PriceBasis, SeriesChoice,
    store::{BarStore, FileStore, Interval, SeriesKey},
    tiingo::Tiingo,
};
use etoro_agent::strategy::{Sma, backtest_sma, sweep_sma};
use etoro_agent::trader::plan as plan_action;
use etoro_agent::types::manual::Numeric;
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
  etoro-agent backtest SYM[,SYM...] [SOURCE] [--fast N] [--slow N]
                       [--spread PCT] [--close-fill] [--sweep] [--trades]
                                               dual moving-average crossover over
                                               stored bars, against buy & hold
  etoro-agent plan    SYM [SOURCE] [--fast N] [--slow N] [--allocation USD]
                                               what the strategy would do right
                                               now, against your live portfolio.
                                               Reads only; places nothing.

dates are YYYY-MM-DD; FROM defaults to five years ago and TO to today.
SOURCE defaults to tiingo.

backtest options:
  --fast N       fast moving-average window in sessions (default 20)
  --slow N       slow window; must exceed the fast one (default 100)
  --spread PCT   quoted spread as a percentage of mid (default 0.02). Half is
                 charged per leg. Get the real figure from `prices SYM`: the
                 observed range across ordinary names is ~50x, so one constant
                 is far too harsh on AAPL and far too kind on a thin mid-cap.
  --close-fill   fill at the deciding close instead of the next open. Optimistic:
                 it hands the strategy every overnight gap following a signal.
  --sweep        print return/volatility across a grid of window pairs
  --trades       print every fill
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
        Command::Backtest(options) => backtest(&options),
        Command::Plan {
            symbol,
            source,
            fast,
            slow,
            allocation,
        } => plan(&symbol, &source, fast, slow, allocation).await,
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

/// Says what the strategy would do right now, and does none of it.
///
/// The only command that reads the live account and mentions orders in the
/// same breath, so it is worth being exact about what it does not do: it
/// places nothing, cancels nothing, and needs no write scope. The hard limits,
/// the cost check and the audit log that have to exist before anything acts on
/// this output are milestone 6.
async fn plan(
    symbol: &str,
    source: &str,
    fast: usize,
    slow: usize,
    allocation: Numeric,
) -> Result<()> {
    let store = FileStore::new(store_root());
    let key = SeriesKey::new(source, symbol, Interval::Daily)?;
    let Some(series) = store.load(&key)? else {
        bail!("no stored series for {symbol} from {source}; run: etoro-agent fetch-bars {symbol}");
    };
    let choice = if series
        .bars
        .iter()
        .any(|bar| bar.total_return_close.is_some())
    {
        SeriesChoice::TotalReturn
    } else {
        SeriesChoice::Reported
    };

    let api_key = std::env::var("ETORO_API_KEY")?;
    let environment = environment()?;
    let client = EtoroClient::new(&api_key, &user_key(environment)?, environment)?;
    client.verify_environment().await?;

    let Some(instrument_id) = client.resolve_symbol(symbol).await? else {
        bail!("{symbol}: no instrument matched that symbol");
    };
    let instrument_id = i32::try_from(instrument_id)
        .with_context(|| format!("instrument id {instrument_id} does not fit an int32"))?;

    let portfolio = client.portfolio().await?;
    let portfolio = portfolio
        .client_portfolio
        .ok_or_else(|| anyhow!("portfolio response omitted clientPortfolio"))?;

    let mut strategy = Sma::new(fast, slow)?;
    let plan = plan_action(
        &portfolio,
        instrument_id,
        &series.bars,
        choice,
        &mut strategy,
        allocation,
    )?;

    let last = series.bars.last().map(|bar| bar.date);
    println!(
        "{} (instrument {instrument_id}) on the {environment} account",
        symbol.to_ascii_uppercase()
    );
    println!(
        "  strategy   sma {fast}/{slow} over {} stored bars, last {}",
        series.bars.len(),
        last.map_or_else(|| "-".to_owned(), |date| date.to_string()),
    );
    println!("  target     {:.0}% of allocation", plan.target * 100.0);
    match &plan.held {
        Some(holding) => println!(
            "  held       position {} - {} units{}",
            holding.position_id,
            holding.units.0,
            holding
                .amount
                .map_or_else(String::new, |amount| format!(", ${}", amount.0)),
        ),
        None => println!("  held       nothing"),
    }
    println!("  allocation ${}", allocation.0);
    println!("\n  -> {}", plan.action.describe());

    // Said plainly, every time. The distance between "here is what I would do"
    // and "I did it" is the entire safety margin at this stage.
    println!(
        "\nNothing was sent. This command only reads; no order path is wired to \
         the command line yet."
    );

    // The store is a snapshot, and a stale one silently plans against last
    // week's prices. Cheap to check, and invisible if it is not checked.
    if let Some(last) = last {
        let age = (Utc::now().date_naive() - last).num_days();
        if age > STALE_BARS_DAYS {
            println!(
                "! the newest stored bar is {age} days old; run: etoro-agent fetch-bars {symbol}"
            );
        }
    }
    Ok(())
}

/// Backtests a dual moving-average crossover over stored bars.
/// Backtests a dual moving-average crossover over stored bars.
///
/// Offline by construction: it reads the local store and nothing else. That is
/// deliberate rather than incidental -- a backtest whose result depends on
/// what a vendor served that afternoon is not reproducible, and the whole
/// value of one is being able to re-run it and get the same answer.
fn backtest(options: &BacktestOptions) -> Result<()> {
    let store = FileStore::new(store_root());
    let costs = CostModel::from_spread(options.spread);
    let mut rows = Vec::new();

    for symbol in &options.symbols {
        let key = SeriesKey::new(&options.source, symbol, Interval::Daily)?;
        // One missing ticker should not abandon the rest of the basket.
        let Some(series) = store.load(&key)? else {
            println!("{symbol}: not in the store; run: etoro-agent fetch-bars {symbol}");
            continue;
        };
        // Total return where the source carries it. On a price-return series
        // a dividend reads as a fall the holder never suffered, and the
        // crossover would trade it.
        let adjusted = series
            .bars
            .iter()
            .any(|bar| bar.total_return_close.is_some());
        let choice = if adjusted {
            SeriesChoice::TotalReturn
        } else {
            SeriesChoice::Reported
        };

        let result = backtest_sma(
            &series.bars,
            choice,
            &costs,
            options.fill,
            options.fast,
            options.slow,
        )?;
        rows.push((symbol.clone(), adjusted, choice, series.bars, result));
    }

    let Some((first_symbol, first_adjusted, _, _, first_result)) = rows.first() else {
        bail!("no stored series to backtest");
    };

    if rows.len() == 1 {
        println!(
            "{} {} ({} prices from {})",
            first_symbol.to_ascii_uppercase(),
            Interval::Daily.as_str(),
            if *first_adjusted {
                "TotalReturn"
            } else {
                "Reported"
            },
            options.source,
        );
        if !first_adjusted {
            println!(
                "! no adjusted closes in this series, so dividends and splits will \
                 move the averages"
            );
        }
        println!();
        print!("{}", first_result.report());
    } else {
        print_basket(&rows);
    }

    if options.trades {
        for (symbol, _, _, _, result) in &rows {
            print_trades(symbol, result);
        }
    }

    if options.sweep {
        for (symbol, _, choice, bars, _) in &rows {
            let sweep = sweep_sma(
                bars,
                *choice,
                &costs,
                options.fill,
                &SWEEP_FAST,
                &SWEEP_SLOW,
            )?;
            println!(
                "\n{} return/volatility by window pair:",
                symbol.to_ascii_uppercase()
            );
            print!("{}", sweep.grid());
        }
    }
    Ok(())
}

/// One row per ticker, for telling an effect from an idiosyncrasy.
fn print_basket(
    rows: &[(
        String,
        bool,
        SeriesChoice,
        Vec<etoro_agent::data::Bar>,
        Backtest,
    )],
) {
    println!(
        "{:<8}{:>9}{:>10}{:>8}{:>9}{:>10}{:>8}{:>8}",
        "symbol", "sessions", "annual", "vol", "ret/vol", "maxDD", "bench", "in mkt"
    );
    let mut any_unadjusted = false;
    let mut beaten = 0;
    for (symbol, adjusted, _, _, result) in rows {
        any_unadjusted |= !adjusted;
        beaten += usize::from(result.beats_benchmark());
        println!(
            "{:<8}{:>9}{:>10}{:>8}{:>9}{:>10}{:>8}{:>8}",
            format!(
                "{}{}",
                symbol.to_ascii_uppercase(),
                if *adjusted { "" } else { "*" }
            ),
            result.segments,
            format!("{:+.2}%", result.strategy.annualised * 100.0),
            format!("{:.1}%", result.strategy.volatility * 100.0),
            ratio(result.strategy.return_over_vol()),
            format!("{:+.2}%", result.max_drawdown * 100.0),
            ratio(result.benchmark.return_over_vol()),
            format!("{:.0}%", result.exposure * 100.0),
        );
    }
    println!(
        "\nbench is buy & hold's return per unit of volatility over the same window.\n\
         beat it in {beaten} of {} names.",
        rows.len()
    );
    if any_unadjusted {
        println!("* no adjusted closes, so dividends and splits move the averages.");
    }
}

fn print_trades(symbol: &str, result: &Backtest) {
    println!(
        "\n{} fills ({}):",
        symbol.to_ascii_uppercase(),
        result.strategy_name
    );
    for trade in &result.trades {
        println!(
            "  {}  {:>5.0}% -> {:>3.0}%  at {:>10.4}  cost {:.4}%",
            trade.date,
            trade.from * 100.0,
            trade.to * 100.0,
            trade.price,
            trade.cost * 100.0,
        );
    }
}

fn ratio(value: f64) -> String {
    if value.is_finite() {
        format!("{value:.2}")
    } else {
        "-".to_owned()
    }
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

/// Which eToro account to act on, read from the environment.
///
/// Required, with no default. A default would have to be one of two things:
/// `demo`, which silently does nothing useful for a real key, or `real`, which
/// silently points at actual money. Neither is a good thing to happen by
/// omission, so the program refuses to guess.
fn environment() -> Result<Environment> {
    let raw = std::env::var("ETORO_ENVIRONMENT").map_err(|_| {
        anyhow!(
            "ETORO_ENVIRONMENT is not set. Add `ETORO_ENVIRONMENT=demo` or \
             `ETORO_ENVIRONMENT=real` to .env -- it must match the environment \
             the API key was issued for, since eToro scopes each key to one."
        )
    })?;
    raw.parse().map_err(Into::into)
}

/// The user key for `environment`, read from a variable named after it.
///
/// One `ETORO_API_KEY` identifies the application and does not change; the
/// *user* key is what eToro scopes to a single environment, so there is one per
/// environment rather than one that gets edited in place.
///
/// Deliberately no fallback to a generic `ETORO_USER_KEY`. A fallback is
/// exactly the hazard this exists to remove: it would let a real key be used
/// against demo paths, and the failure would arrive as an unexplained 403
/// instead of a sentence naming the variable to set.
fn user_key(environment: Environment) -> Result<String> {
    let name = match environment {
        Environment::Demo => "ETORO_DEMO_USER_KEY",
        Environment::Real => "ETORO_REAL_USER_KEY",
    };
    std::env::var(name).map_err(|_| {
        anyhow!(
            "{name} is not set, and ETORO_ENVIRONMENT={environment} needs it. \
             eToro issues one user key per environment, so keep both in .env: \
             ETORO_DEMO_USER_KEY and ETORO_REAL_USER_KEY."
        )
    })
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
    let dump_responses = env_flag("ETORO_DUMP_RESPONSES");

    let environment = environment()?;
    let client = EtoroClient::new(&api_key, &user_key(environment)?, environment)?;

    // Before anything else, and before the first write path ever exists: keys
    // are scoped to one environment, and a mismatch is far cheaper to learn
    // about here than as a 403 on an order.
    let me = client.verify_environment().await?;
    println!(
        "Authenticated as {} on the {environment} account.",
        me.username
    );
    if dump_responses {
        dump_private("me_response.json", &me)?;
    }

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

/// Default crossover windows, in sessions.
const DEFAULT_FAST: usize = 20;
const DEFAULT_SLOW: usize = 100;

/// Assumed spread, as a percentage of mid, when none is given.
///
/// A placeholder between the extremes actually measured -- 0.003% on AAPL,
/// 0.163% on a watchlist mid-cap -- and wrong for every specific instrument,
/// which is why the report prints what it charged rather than leaving it
/// implied.
const DEFAULT_SPREAD_PCT: f64 = 0.02;

/// How stale the newest stored bar may be before `plan` says so.
///
/// Three days covers an ordinary weekend plus a public holiday, so a warning
/// means something is actually wrong rather than that it is Sunday.
const STALE_BARS_DAYS: i64 = 3;

/// Cash committed to a new position when `--allocation` is not given, in USD.
///
/// The low end of the range this project set out to trade. Printed on every
/// plan rather than assumed, because it is the number that decides how much
/// money an eventual order moves.
const DEFAULT_ALLOCATION_USD: u32 = 100;

/// Window pairs the sweep walks, following the reference harness.
const SWEEP_FAST: [usize; 5] = [5, 10, 20, 40, 60];
const SWEEP_SLOW: [usize; 5] = [50, 100, 150, 200, 250];

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
    /// Boxed into its own struct because the option list is long enough that
    /// inlining it would swamp the other variants.
    Backtest(BacktestOptions),
    Plan {
        symbol: String,
        source: String,
        fast: usize,
        slow: usize,
        /// Cash to put into a new position, in USD.
        allocation: Numeric,
    },
}

#[derive(Debug, PartialEq)]
struct BacktestOptions {
    symbols: Vec<String>,
    source: String,
    fast: usize,
    slow: usize,
    /// Quoted spread as a fraction of mid, halved per leg by the cost model.
    spread: f64,
    fill: FillPrice,
    sweep: bool,
    trades: bool,
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
            Some((verb, rest)) if verb == "backtest" => {
                let mut options = BacktestOptions {
                    symbols: Vec::new(),
                    source: DEFAULT_SOURCE.to_owned(),
                    fast: DEFAULT_FAST,
                    slow: DEFAULT_SLOW,
                    spread: DEFAULT_SPREAD_PCT / 100.0,
                    fill: FillPrice::NextOpen,
                    sweep: false,
                    trades: false,
                };
                let mut positional = Vec::new();
                let mut args = rest.iter();
                while let Some(arg) = args.next() {
                    // Every value-taking flag reports the flag it belongs to
                    // rather than "missing argument", so a truncated command
                    // line says which part was truncated.
                    let mut value = |flag: &str| -> Result<String> {
                        args.next()
                            .cloned()
                            .ok_or_else(|| anyhow!("{flag} needs a value\n\n{USAGE}"))
                    };
                    match arg.as_str() {
                        "--fast" => options.fast = parse_window(&value("--fast")?, "--fast")?,
                        "--slow" => options.slow = parse_window(&value("--slow")?, "--slow")?,
                        "--spread" => options.spread = parse_spread(&value("--spread")?)?,
                        "--close-fill" => options.fill = FillPrice::SameClose,
                        "--sweep" => options.sweep = true,
                        "--trades" => options.trades = true,
                        other if other.starts_with("--") => {
                            bail!("unknown option {other:?}\n\n{USAGE}")
                        }
                        other => positional.push(other.to_owned()),
                    }
                }
                let Some(symbols) = positional.first() else {
                    bail!("backtest needs at least one symbol\n\n{USAGE}");
                };
                options.symbols = parse_symbols(symbols);
                if let Some(source) = positional.get(1) {
                    options.source.clone_from(source);
                }
                Ok(Self::Backtest(options))
            }
            Some((verb, rest)) if verb == "plan" => {
                let (mut fast, mut slow) = (DEFAULT_FAST, DEFAULT_SLOW);
                let mut allocation = Numeric(DEFAULT_ALLOCATION_USD.into());
                let mut positional = Vec::new();
                let mut args = rest.iter();
                while let Some(arg) = args.next() {
                    let mut value = |flag: &str| -> Result<String> {
                        args.next()
                            .cloned()
                            .ok_or_else(|| anyhow!("{flag} needs a value\n\n{USAGE}"))
                    };
                    match arg.as_str() {
                        "--fast" => fast = parse_window(&value("--fast")?, "--fast")?,
                        "--slow" => slow = parse_window(&value("--slow")?, "--slow")?,
                        "--allocation" => allocation = parse_allocation(&value("--allocation")?)?,
                        other if other.starts_with("--") => {
                            bail!("unknown option {other:?}\n\n{USAGE}")
                        }
                        other => positional.push(other.to_owned()),
                    }
                }
                let Some(symbol) = positional.first() else {
                    bail!("plan needs a symbol\n\n{USAGE}");
                };
                Ok(Self::Plan {
                    symbol: symbol.clone(),
                    source: positional
                        .get(1)
                        .cloned()
                        .unwrap_or_else(|| DEFAULT_SOURCE.to_owned()),
                    fast,
                    slow,
                    allocation,
                })
            }
            // A bare symbol list used to mean "price these". Rejecting it is
            // better than guessing, now that a verb could also be a ticker.
            Some((other, _)) => bail!("unknown command {other:?}\n\n{USAGE}"),
        }
    }
}

/// A moving-average window, which is a count of sessions.
fn parse_window(text: &str, flag: &str) -> Result<usize> {
    let window: usize = text
        .parse()
        .with_context(|| format!("{flag} takes a whole number of sessions, not {text:?}"))?;
    if window == 0 {
        bail!("{flag} must span at least one session");
    }
    Ok(window)
}

/// A spread quoted as a percentage of mid, stored as a fraction.
///
/// Rejecting negatives and anything above a whole percent: a spread wider than
/// 1% of mid is not an instrument this harness should be pretending to trade,
/// and a typo like `--spread 20` meaning twenty basis points would otherwise
/// silently charge twenty percent and produce a plausible-looking disaster.
fn parse_spread(text: &str) -> Result<f64> {
    let percent: f64 = text
        .parse()
        .with_context(|| format!("--spread takes a percentage, not {text:?}"))?;
    if !percent.is_finite() || percent < 0.0 {
        bail!("--spread must be a percentage of mid that is zero or more, got {percent}");
    }
    if percent > 1.0 {
        bail!(
            "--spread is a percentage of mid, and {percent}% is wider than any instrument \
             worth backtesting; 0.02 means two basis points"
        );
    }
    Ok(percent / 100.0)
}

/// Cash for one position, in USD.
///
/// Parsed as a decimal and never through `f64`: this is the number that
/// becomes an order amount, and the whole wire layer exists to keep such
/// numbers exact.
fn parse_allocation(text: &str) -> Result<Numeric> {
    let amount: rust_decimal::Decimal = text
        .parse()
        .with_context(|| format!("--allocation takes a USD amount, not {text:?}"))?;
    if amount <= rust_decimal::Decimal::ZERO {
        bail!("--allocation must be greater than zero, got {amount}");
    }
    Ok(Numeric(amount))
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

    fn parse(args: &[&str]) -> Result<Command> {
        Command::parse(args.iter().map(|a| (*a).to_owned()).collect())
    }

    fn backtest_options(args: &[&str]) -> BacktestOptions {
        match parse(args).unwrap() {
            Command::Backtest(options) => options,
            other => panic!("expected a backtest, got {other:?}"),
        }
    }

    #[test]
    fn backtest_defaults_to_the_pessimistic_fill_and_a_stated_spread() {
        let options = backtest_options(&["backtest", "AAPL"]);
        assert_eq!(options.symbols, ["AAPL"]);
        assert_eq!(options.source, "tiingo");
        assert_eq!((options.fast, options.slow), (20, 100));
        // Filling at the next open cannot capture the gap after a signal,
        // which is the assumption to make by default rather than opt into.
        assert_eq!(options.fill, FillPrice::NextOpen);
        assert!(!options.sweep && !options.trades);
        assert!((options.spread - 0.0002).abs() < 1e-12, "0.02% of mid");
    }

    #[test]
    fn backtest_reads_every_option() {
        let options = backtest_options(&[
            "backtest",
            "AAPL,MSFT",
            "tiingo",
            "--fast",
            "5",
            "--slow",
            "50",
            "--spread",
            "0.116",
            "--close-fill",
            "--sweep",
            "--trades",
        ]);
        assert_eq!(options.symbols, ["AAPL", "MSFT"]);
        assert_eq!((options.fast, options.slow), (5, 50));
        assert_eq!(options.fill, FillPrice::SameClose);
        assert!(options.sweep && options.trades);
        assert!((options.spread - 0.00116).abs() < 1e-12, "MBLY's spread");
    }

    #[test]
    fn a_flag_missing_its_value_names_the_flag() {
        let error = parse(&["backtest", "AAPL", "--slow"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("--slow needs a value"), "{error}");
    }

    #[test]
    fn a_spread_that_is_probably_basis_points_is_refused() {
        // `--spread 20` meaning twenty basis points would otherwise charge
        // twenty percent and produce a plausible-looking disaster.
        let error = parse(&["backtest", "AAPL", "--spread", "20"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("percentage of mid"), "{error}");

        assert!(parse(&["backtest", "AAPL", "--spread", "-1"]).is_err());
        assert!(parse(&["backtest", "AAPL", "--spread", "wide"]).is_err());
        // Zero is a real choice, and one the report calls out as frictionless.
        assert!(parse(&["backtest", "AAPL", "--spread", "0"]).is_ok());
    }

    #[test]
    fn a_zero_length_window_is_refused_before_the_engine_sees_it() {
        let error = parse(&["backtest", "AAPL", "--fast", "0"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("at least one session"), "{error}");
        assert!(parse(&["backtest", "AAPL", "--fast", "2.5"]).is_err());
    }

    #[test]
    fn backtest_needs_a_symbol_and_rejects_unknown_options() {
        assert!(parse(&["backtest"]).is_err());
        let error = parse(&["backtest", "AAPL", "--turbo"])
            .unwrap_err()
            .to_string();
        assert!(error.contains("--turbo"), "{error}");
    }

    #[test]
    fn plan_defaults_to_the_smallest_allocation_this_project_set_out_to_trade() {
        let Command::Plan {
            symbol,
            source,
            fast,
            slow,
            allocation,
        } = parse(&["plan", "AAPL"]).unwrap()
        else {
            panic!("expected a plan");
        };
        assert_eq!((symbol.as_str(), source.as_str()), ("AAPL", "tiingo"));
        assert_eq!((fast, slow), (20, 100));
        assert_eq!(allocation.0.to_string(), "100");
    }

    #[test]
    fn an_allocation_is_parsed_as_a_decimal_and_never_through_a_float() {
        let Command::Plan { allocation, .. } =
            parse(&["plan", "AAPL", "--allocation", "250.75"]).unwrap()
        else {
            panic!("expected a plan");
        };
        // Exactly, scale included: this becomes an order amount.
        assert_eq!(allocation.0.to_string(), "250.75");

        // Money that could not buy anything is not a size.
        assert!(parse(&["plan", "AAPL", "--allocation", "0"]).is_err());
        assert!(parse(&["plan", "AAPL", "--allocation", "-5"]).is_err());
        assert!(parse(&["plan", "AAPL", "--allocation", "lots"]).is_err());
    }

    #[test]
    fn plan_needs_a_symbol_and_rejects_unknown_options() {
        assert!(parse(&["plan"]).is_err());
        assert!(parse(&["plan", "AAPL", "--execute"]).is_err());
        assert!(parse(&["plan", "AAPL", "--allocation"]).is_err());
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
