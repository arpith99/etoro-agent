//! Local persistence for bar series.
//!
//! Newline-delimited JSON, one file per `(source, symbol, interval)`, replaced
//! atomically. The layout is the point: a series has exactly one
//! [`PriceBasis`], and putting the source in the path makes mixing eToro's
//! split-adjusted bars with Tiingo's as-traded ones into a single series
//! impossible rather than merely discouraged.
//!
//! Persisting at all is not optional. eToro serves a rolling window with no
//! date range, so every bar not written down becomes permanently unreachable
//! as the window advances.
//!
//! Why not SQLite: at daily resolution a symbol is about a thousand rows a
//! decade, where indexes and partial updates buy nothing, and SQLite has no
//! decimal type -- storing prices as `REAL` would silently reintroduce the f64
//! loss that [`Numeric`](crate::types::manual::Numeric) exists to prevent.
//! Revisit at intraday resolution, where a symbol-year is ~100k rows.

use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{Bar, PriceBasis};

/// Bar period. The store treats these as opaque directory names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Interval {
    Daily,
    Weekly,
    Monthly,
}

impl Interval {
    /// The directory-name form, which is also what a caller prints.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Daily => "1d",
            Self::Weekly => "1w",
            Self::Monthly => "1mo",
        }
    }
}

/// Identifies one stored series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeriesKey {
    source: String,
    symbol: String,
    interval: Interval,
}

impl SeriesKey {
    /// Both `source` and `symbol` become path components, so they are
    /// validated rather than trusted: a `..` or a separator would let a
    /// caller-supplied ticker write outside the store root.
    ///
    /// The symbol is upper-cased because a case-insensitive filesystem would
    /// otherwise let `aapl` and `AAPL` collide into one file while the program
    /// believed they were two series.
    pub fn new(source: &str, symbol: &str, interval: Interval) -> Result<Self, StoreError> {
        let safe = |value: &str| {
            !value.is_empty()
                && value
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
        };
        if !safe(source) || !safe(symbol) {
            return Err(StoreError::InvalidKey {
                detail: format!("source {source:?} and symbol {symbol:?} must be path-safe"),
            });
        }
        Ok(Self {
            source: source.to_ascii_lowercase(),
            symbol: symbol.to_ascii_uppercase(),
            interval,
        })
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn interval(&self) -> Interval {
        self.interval
    }

    fn dir(&self, root: &Path) -> PathBuf {
        root.join(&self.source).join(&self.symbol)
    }

    fn bars_path(&self, root: &Path) -> PathBuf {
        self.dir(root)
            .join(format!("{}.ndjson", self.interval.as_str()))
    }

    fn meta_path(&self, root: &Path) -> PathBuf {
        self.dir(root)
            .join(format!("{}.meta.json", self.interval.as_str()))
    }
}

/// A stored series together with what its prices mean.
///
/// Returning bars without the basis would recreate the problem this layer
/// exists to remove: an unlabelled price series that looks usable and is not.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub basis: PriceBasis,
    pub bars: Vec<Bar>,
}

/// What a merge actually changed.
///
/// Reported rather than inferred, so "re-running does not duplicate rows" is
/// something a caller can assert instead of hope for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MergeOutcome {
    pub added: usize,
    pub updated: usize,
    pub unchanged: usize,
    pub total: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("store I/O failed at {path}: {detail}")]
    Io { path: String, detail: String },

    #[error("stored series at {path} could not be read: {detail}")]
    Corrupt { path: String, detail: String },

    /// Refusing to mix conventions in one series. Silently accepting this
    /// would produce a file whose prices mean two different things, which no
    /// later reader could untangle.
    #[error("series at {path} holds {stored:?} prices; refusing to merge {incoming:?}")]
    BasisMismatch {
        path: String,
        stored: PriceBasis,
        incoming: PriceBasis,
    },

    #[error("invalid series key: {detail}")]
    InvalidKey { detail: String },
}

/// Persistence for bar series.
///
/// A trait because milestone 3 wants an in-memory implementation for tests
/// that neither touches the filesystem nor needs cleaning up.
pub trait BarStore {
    /// Returns `None` when the series has never been written.
    fn load(&self, key: &SeriesKey) -> Result<Option<Series>, StoreError>;

    /// Inserts `bars`, replacing any that share a date, and reports what
    /// changed. Re-merging identical bars is a no-op by construction.
    fn merge(
        &self,
        key: &SeriesKey,
        basis: PriceBasis,
        bars: &[Bar],
    ) -> Result<MergeOutcome, StoreError>;
}

/// Newline-delimited JSON under a root directory.
#[derive(Debug, Clone)]
pub struct FileStore {
    root: PathBuf,
}

/// Sidecar recording what the neighbouring `.ndjson` file's prices mean.
#[derive(Debug, Serialize, Deserialize)]
struct SeriesMeta {
    source: String,
    symbol: String,
    interval: Interval,
    basis: PriceBasis,
}

impl FileStore {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Writes `contents` to `path` without ever leaving a partial file there.
    ///
    /// A crash part-way through an in-place rewrite would truncate a series
    /// that took real time to accumulate -- and for eToro-sourced bars, the
    /// lost portion may no longer be fetchable. The temporary file is created
    /// in the destination directory so the rename stays within one filesystem,
    /// which is what makes it atomic.
    fn write_atomically(path: &Path, contents: &str) -> Result<(), StoreError> {
        let io = |detail: std::io::Error| StoreError::Io {
            path: path.display().to_string(),
            detail: detail.to_string(),
        };
        let parent = path.parent().unwrap_or(Path::new("."));
        fs::create_dir_all(parent).map_err(io)?;

        let temp = path.with_extension("tmp");
        {
            let mut file = fs::File::create(&temp).map_err(io)?;
            file.write_all(contents.as_bytes()).map_err(io)?;
            // Without this the rename can be durable while the contents are
            // not, leaving an empty file after a power loss.
            file.sync_all().map_err(io)?;
        }
        fs::rename(&temp, path).map_err(io)
    }

    fn read_meta(&self, key: &SeriesKey) -> Result<Option<SeriesMeta>, StoreError> {
        let path = key.meta_path(&self.root);
        match fs::read_to_string(&path) {
            Ok(text) => {
                serde_json::from_str(&text)
                    .map(Some)
                    .map_err(|error| StoreError::Corrupt {
                        path: path.display().to_string(),
                        detail: error.to_string(),
                    })
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(StoreError::Io {
                path: path.display().to_string(),
                detail: error.to_string(),
            }),
        }
    }
}

impl BarStore for FileStore {
    fn load(&self, key: &SeriesKey) -> Result<Option<Series>, StoreError> {
        let Some(meta) = self.read_meta(key)? else {
            return Ok(None);
        };
        let path = key.bars_path(&self.root);
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            // A sidecar with no bars file is a half-written store, not an
            // empty series; say so rather than pretending it is fine.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(StoreError::Corrupt {
                    path: path.display().to_string(),
                    detail: "metadata exists but the bars file is missing".to_owned(),
                });
            }
            Err(error) => {
                return Err(StoreError::Io {
                    path: path.display().to_string(),
                    detail: error.to_string(),
                });
            }
        };

        let mut bars = Vec::new();
        for (number, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            let bar: Bar = serde_json::from_str(line).map_err(|error| StoreError::Corrupt {
                path: path.display().to_string(),
                detail: format!("line {}: {error}", number + 1),
            })?;
            bars.push(bar);
        }
        Ok(Some(Series {
            basis: meta.basis,
            bars,
        }))
    }

    fn merge(
        &self,
        key: &SeriesKey,
        basis: PriceBasis,
        bars: &[Bar],
    ) -> Result<MergeOutcome, StoreError> {
        if let Some(meta) = self.read_meta(key)?
            && meta.basis != basis
        {
            return Err(StoreError::BasisMismatch {
                path: key.meta_path(&self.root).display().to_string(),
                stored: meta.basis,
                incoming: basis,
            });
        }

        let existing = self
            .load(key)?
            .map(|series| series.bars)
            .unwrap_or_default();
        // A BTreeMap keyed by date does the deduplication and the ordering in
        // one step, which is why the stored file is always sorted without a
        // separate sort.
        let mut merged: BTreeMap<NaiveDate, Bar> =
            existing.into_iter().map(|bar| (bar.date, bar)).collect();

        let mut outcome = MergeOutcome::default();
        for bar in bars {
            match merged.get(&bar.date) {
                // A vendor revising a bar is normal; take the newer value but
                // do not report churn when nothing actually changed.
                Some(stored) if stored == bar => outcome.unchanged += 1,
                Some(_) => {
                    outcome.updated += 1;
                    merged.insert(bar.date, bar.clone());
                }
                None => {
                    outcome.added += 1;
                    merged.insert(bar.date, bar.clone());
                }
            }
        }
        outcome.total = merged.len();

        let mut body = String::new();
        for bar in merged.values() {
            let line = serde_json::to_string(bar).map_err(|error| StoreError::Corrupt {
                path: key.bars_path(&self.root).display().to_string(),
                detail: error.to_string(),
            })?;
            body.push_str(&line);
            body.push('\n');
        }
        Self::write_atomically(&key.bars_path(&self.root), &body)?;

        let meta = SeriesMeta {
            source: key.source.clone(),
            symbol: key.symbol.clone(),
            interval: key.interval,
            basis,
        };
        let meta_text =
            serde_json::to_string_pretty(&meta).map_err(|error| StoreError::Corrupt {
                path: key.meta_path(&self.root).display().to_string(),
                detail: error.to_string(),
            })?;
        // Bars first, metadata second: an interrupted merge then leaves a
        // series that reads as absent rather than one that claims a basis for
        // bars it does not have.
        Self::write_atomically(&key.meta_path(&self.root), &meta_text)?;

        Ok(outcome)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::manual::Numeric;

    /// A store root that removes itself, so tests leave nothing behind.
    struct TempRoot(PathBuf);

    impl TempRoot {
        fn new() -> Self {
            Self(
                std::env::temp_dir()
                    .join(format!("etoro-agent-store-test-{}", uuid::Uuid::new_v4())),
            )
        }

        fn store(&self) -> FileStore {
            FileStore::new(&self.0)
        }
    }

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn key() -> SeriesKey {
        SeriesKey::new("tiingo", "AAPL", Interval::Daily).unwrap()
    }

    fn bar(date: &str, close: &str) -> Bar {
        Bar {
            date: date.parse().unwrap(),
            open: Numeric(close.parse().unwrap()),
            high: Numeric(close.parse().unwrap()),
            low: Numeric(close.parse().unwrap()),
            close: Numeric(close.parse().unwrap()),
            volume: None,
            total_return_close: None,
            dividend_cash: None,
            split_factor: None,
            split_adjusted_volume: None,
        }
    }

    #[test]
    fn a_series_that_was_never_written_reads_as_absent_not_empty() {
        let root = TempRoot::new();
        assert!(root.store().load(&key()).unwrap().is_none());
    }

    #[test]
    fn bars_survive_a_round_trip_with_their_basis() {
        let root = TempRoot::new();
        let store = root.store();
        let bars = [bar("2026-08-26", "100.5"), bar("2026-08-27", "101.0")];

        let outcome = store.merge(&key(), PriceBasis::AsTraded, &bars).unwrap();
        assert_eq!(outcome.added, 2);
        assert_eq!(outcome.total, 2);

        let series = store.load(&key()).unwrap().unwrap();
        // Losing the basis would leave an unlabelled price series, which is
        // the failure this whole layer exists to prevent.
        assert_eq!(series.basis, PriceBasis::AsTraded);
        assert_eq!(series.bars, bars);
    }

    #[test]
    fn re_merging_identical_bars_does_not_duplicate_them() {
        // The milestone's acceptance criterion, asserted rather than assumed.
        let root = TempRoot::new();
        let store = root.store();
        let bars = [bar("2026-08-26", "100.5"), bar("2026-08-27", "101.0")];

        store.merge(&key(), PriceBasis::AsTraded, &bars).unwrap();
        let second = store.merge(&key(), PriceBasis::AsTraded, &bars).unwrap();

        assert_eq!(second.unchanged, 2);
        assert_eq!(second.added, 0);
        assert_eq!(second.updated, 0);
        assert_eq!(second.total, 2);
        assert_eq!(store.load(&key()).unwrap().unwrap().bars.len(), 2);
    }

    #[test]
    fn a_revised_bar_replaces_its_predecessor_and_is_reported_as_an_update() {
        let root = TempRoot::new();
        let store = root.store();
        store
            .merge(&key(), PriceBasis::AsTraded, &[bar("2026-08-26", "100.5")])
            .unwrap();

        // Vendors do restate prices; the newer value wins.
        let outcome = store
            .merge(&key(), PriceBasis::AsTraded, &[bar("2026-08-26", "100.75")])
            .unwrap();

        assert_eq!((outcome.added, outcome.updated, outcome.total), (0, 1, 1));
        let series = store.load(&key()).unwrap().unwrap();
        assert_eq!(series.bars[0].close.to_string(), "100.75");
    }

    #[test]
    fn bars_are_stored_in_date_order_whatever_order_they_arrive_in() {
        let root = TempRoot::new();
        let store = root.store();
        store
            .merge(
                &key(),
                PriceBasis::AsTraded,
                &[bar("2026-08-28", "3"), bar("2026-08-26", "1")],
            )
            .unwrap();
        store
            .merge(&key(), PriceBasis::AsTraded, &[bar("2026-08-27", "2")])
            .unwrap();

        let dates: Vec<String> = store
            .load(&key())
            .unwrap()
            .unwrap()
            .bars
            .iter()
            .map(|bar| bar.date.to_string())
            .collect();
        assert_eq!(dates, ["2026-08-26", "2026-08-27", "2026-08-28"]);
    }

    #[test]
    fn merging_a_different_basis_into_an_existing_series_is_refused() {
        let root = TempRoot::new();
        let store = root.store();
        store
            .merge(&key(), PriceBasis::AsTraded, &[bar("2026-08-26", "100")])
            .unwrap();

        // Accepting this would produce one file whose prices mean two
        // different things, which no later reader could untangle.
        let error = store
            .merge(&key(), PriceBasis::TotalReturn, &[bar("2026-08-27", "90")])
            .unwrap_err();
        assert!(matches!(error, StoreError::BasisMismatch { .. }));
    }

    #[test]
    fn exact_decimals_survive_storage() {
        let root = TempRoot::new();
        let store = root.store();
        let precise = bar("2026-08-26", "100.0000000000000000000001");
        store
            .merge(&key(), PriceBasis::AsTraded, &[precise])
            .unwrap();

        // The reason this store is not SQLite: a REAL column would round this
        // away silently.
        let series = store.load(&key()).unwrap().unwrap();
        assert_eq!(
            series.bars[0].close.to_string(),
            "100.0000000000000000000001"
        );
    }

    #[test]
    fn keys_that_could_escape_the_store_root_are_rejected() {
        assert!(SeriesKey::new("tiingo", "../../etc", Interval::Daily).is_err());
        assert!(SeriesKey::new("../..", "AAPL", Interval::Daily).is_err());
        assert!(SeriesKey::new("tiingo", "", Interval::Daily).is_err());
        assert!(SeriesKey::new("tiingo", "BRK-B", Interval::Daily).is_ok());
    }

    #[test]
    fn symbol_case_cannot_split_one_series_into_two() {
        // On a case-insensitive filesystem the two spellings would share a
        // file while the program believed they were separate series.
        let lower = SeriesKey::new("tiingo", "aapl", Interval::Daily).unwrap();
        assert_eq!(lower, key());
    }
}
