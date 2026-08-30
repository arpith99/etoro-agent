use std::path::Path;

use anyhow::Result;
use etoro_agent::client::EtoroClient;
use serde::Serialize;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
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

    // Symbols come in as one comma-separated argument. Resolution is one call
    // per symbol -- the search filter takes a single value -- but pricing is a
    // single call for the whole set, which is the batching that matters:
    // `rates` accepts up to 100 IDs at once.
    let symbols = requested_symbols();
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

/// Symbol priced when none is given on the command line.
const DEFAULT_SYMBOL: &str = "AAPL";

/// Parses the optional `AAPL,TSLA,MSFT` argument.
///
/// Duplicates are dropped so a repeated symbol does not cost an extra
/// resolution call, and order is preserved so the output matches what was
/// asked for. Case is left as typed: symbol matching is case-insensitive, and
/// echoing the input back unchanged is less confusing than correcting it.
fn requested_symbols() -> Vec<String> {
    parse_symbols(
        &std::env::args()
            .nth(1)
            .unwrap_or_else(|| DEFAULT_SYMBOL.to_owned()),
    )
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
