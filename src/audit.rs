//! An append-only record of what the agent intended and what happened.
//!
//! Two jobs, and the second is the one that justifies the design.
//!
//! It is a **record**: after an unattended run, the only account of why a
//! position exists is what was written down at the time. So entries are
//! appended and never rewritten, and a refusal is logged as loudly as a
//! submission — "the agent did nothing today" and "the agent was stopped from
//! doing something twelve times" look identical in a portfolio and are not the
//! same situation.
//!
//! It is also **state**. The daily order cap is enforced by counting what this
//! file says was submitted, which means the cap survives a restart, a crash, or
//! two copies of the program running at once. A counter held in memory would
//! reset at exactly the moment it mattered most.
//!
//! Written with `O_APPEND` and mode `0600`. It names instruments, amounts and
//! times, which is account activity rather than market data.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::types::manual::Numeric;

/// One thing that happened.
/// Externally tagged -- `{"submitted": {...}}` -- rather than internally
/// tagged, and deliberately.
///
/// `serde_json`'s `arbitrary_precision`, which this crate depends on to keep
/// prices exact, is incompatible with every serde representation that buffers
/// a value before matching on it: `flatten`, `tag = "..."`, `tag`+`content`,
/// and `untagged` all route through an internal map form, after which a plain
/// `f64` field fails to deserialize with "invalid type: map, expected f64".
/// External tagging is the one representation that never buffers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Event {
    /// A decision was made and not acted on -- either because nothing needed
    /// doing, or because the run was a dry one.
    Planned {
        symbol: String,
        instrument_id: i32,
        target: f64,
        action: String,
    },
    /// A limit, a rail, or a refusal in the decision layer stopped an action.
    ///
    /// The most important variant to have. A refused action leaves no trace
    /// anywhere else: no order, no position, no balance change.
    Refused {
        symbol: String,
        instrument_id: i32,
        action: String,
        reason: String,
    },
    /// An order was sent. Written *before* the response is known.
    Submitted {
        symbol: String,
        instrument_id: i32,
        action: String,
        /// The `x-request-id`, which is also the recovery handle.
        reference_id: uuid::Uuid,
        amount: Option<Numeric>,
    },
    /// What became of a submitted order.
    Settled {
        reference_id: uuid::Uuid,
        order_id: Option<i64>,
        status: Option<String>,
        detail: Option<String>,
    },
}

/// A timestamped entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Record {
    pub at: DateTime<Utc>,
    /// `demo` or `real`. Recorded per entry rather than per file so that a log
    /// read months later cannot be misattributed to the wrong account.
    pub environment: String,
    /// Nested rather than `#[serde(flatten)]`; see [`Event`] for why.
    pub event: Event,
}

impl Record {
    pub fn now(environment: &str, event: Event) -> Self {
        Self {
            at: Utc::now(),
            environment: environment.to_owned(),
            event,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AuditError {
    #[error("audit log at {path} could not be written: {detail}")]
    Io { path: String, detail: String },

    /// A line that does not parse.
    ///
    /// Reported rather than skipped: the daily order cap is computed from this
    /// file, so a line that cannot be read is a line whose orders cannot be
    /// counted, and quietly counting fewer orders than were placed is the one
    /// failure a cap must not have.
    #[error("audit log at {path} has an unreadable entry on line {line}: {detail}")]
    Corrupt {
        path: String,
        line: usize,
        detail: String,
    },
}

/// Newline-delimited JSON, appended to and never rewritten.
#[derive(Debug, Clone)]
pub struct AuditLog {
    path: PathBuf,
}

impl AuditLog {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Appends one entry, creating the file if needed.
    ///
    /// `O_APPEND` rather than seek-then-write, so two processes writing at once
    /// interleave whole lines instead of overwriting each other -- which is the
    /// case that matters, since the cap this file feeds exists precisely to
    /// survive a second copy of the program.
    pub fn append(&self, record: &Record) -> Result<(), AuditError> {
        let io = |detail: std::io::Error| AuditError::Io {
            path: self.path.display().to_string(),
            detail: detail.to_string(),
        };
        if let Some(parent) = self
            .path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            fs::create_dir_all(parent).map_err(io)?;
        }

        let mut line = serde_json::to_string(record).map_err(|error| AuditError::Io {
            path: self.path.display().to_string(),
            detail: error.to_string(),
        })?;
        line.push('\n');

        let mut file = self.open_appending()?;
        file.write_all(line.as_bytes()).map_err(io)?;
        // Without this an entry can be lost to a crash between the write and
        // the flush -- and the entry most likely to be written just before a
        // crash is the one recording a submitted order.
        file.sync_all().map_err(io)
    }

    #[cfg(unix)]
    fn open_appending(&self) -> Result<fs::File, AuditError> {
        use std::os::unix::fs::OpenOptionsExt;
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .mode(0o600)
            .open(&self.path)
            .map_err(|detail| AuditError::Io {
                path: self.path.display().to_string(),
                detail: detail.to_string(),
            })
    }

    #[cfg(not(unix))]
    fn open_appending(&self) -> Result<fs::File, AuditError> {
        fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)
            .map_err(|detail| AuditError::Io {
                path: self.path.display().to_string(),
                detail: detail.to_string(),
            })
    }

    /// Every entry, oldest first. An absent file reads as empty.
    pub fn read(&self) -> Result<Vec<Record>, AuditError> {
        let text = match fs::read_to_string(&self.path) {
            Ok(text) => text,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(error) => {
                return Err(AuditError::Io {
                    path: self.path.display().to_string(),
                    detail: error.to_string(),
                });
            }
        };

        text.lines()
            .enumerate()
            .filter(|(_, line)| !line.trim().is_empty())
            .map(|(index, line)| {
                serde_json::from_str(line).map_err(|error| AuditError::Corrupt {
                    path: self.path.display().to_string(),
                    line: index + 1,
                    detail: error.to_string(),
                })
            })
            .collect()
    }

    /// How many orders were **submitted** on `date`, in `environment`.
    ///
    /// The number the daily cap is checked against. Counts submissions rather
    /// than settlements on purpose: an order that was sent and whose outcome
    /// was never recorded still consumed a slot, and may well have filled.
    ///
    /// Counts **distinct `reference_id`s**, not records. A retried write reuses
    /// its id -- that is the whole idempotency contract -- so it is one order
    /// however many times it was sent, and logging each attempt is what makes
    /// the log useful afterwards. Counting records instead would let a flaky
    /// connection exhaust the day's budget without a single extra position
    /// being opened.
    ///
    /// UTC, matching the timestamps, which does not align with any exchange's
    /// trading day. That is deliberate -- the cap is a brake on this program,
    /// not a description of a session, and a boundary that never shifts is
    /// easier to reason about than one that does twice a year.
    pub fn submissions_on(&self, date: NaiveDate, environment: &str) -> Result<usize, AuditError> {
        let mut orders = std::collections::HashSet::new();
        for record in self.read()? {
            if record.at.date_naive() != date || record.environment != environment {
                continue;
            }
            if let Event::Submitted { reference_id, .. } = record.event {
                orders.insert(reference_id);
            }
        }
        Ok(orders.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TempLog(PathBuf);

    impl TempLog {
        fn new() -> Self {
            Self(
                std::env::temp_dir()
                    .join(format!("etoro-agent-audit-{}.ndjson", uuid::Uuid::new_v4())),
            )
        }

        fn log(&self) -> AuditLog {
            AuditLog::new(&self.0)
        }
    }

    impl Drop for TempLog {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.0);
        }
    }

    fn submitted(reference: uuid::Uuid) -> Event {
        Event::Submitted {
            symbol: "AAPL".to_owned(),
            instrument_id: 1001,
            action: "OPEN".to_owned(),
            reference_id: reference,
            amount: Some(Numeric("100".parse().unwrap())),
        }
    }

    #[test]
    fn an_absent_log_reads_as_empty_rather_than_failing() {
        let temp = TempLog::new();
        assert_eq!(temp.log().read().unwrap(), Vec::new());
        assert_eq!(
            temp.log()
                .submissions_on(Utc::now().date_naive(), "demo")
                .unwrap(),
            0
        );
    }

    #[test]
    fn entries_are_appended_rather_than_replacing_what_came_before() {
        let temp = TempLog::new();
        let log = temp.log();
        for index in 0..3 {
            log.append(&Record::now(
                "demo",
                Event::Planned {
                    symbol: "AAPL".to_owned(),
                    instrument_id: 1001,
                    target: f64::from(index % 2),
                    action: "hold".to_owned(),
                },
            ))
            .unwrap();
        }
        assert_eq!(log.read().unwrap().len(), 3, "nothing was overwritten");
    }

    #[test]
    fn a_record_round_trips_through_the_file() {
        let temp = TempLog::new();
        let reference = uuid::Uuid::new_v4();
        let record = Record::now("demo", submitted(reference));
        temp.log().append(&record).unwrap();

        let read = temp.log().read().unwrap();
        assert_eq!(read.len(), 1);
        assert_eq!(read[0], record);
        // The amount keeps its scale here too: this file is the account of
        // what was sent, so it has to agree with what was sent.
        assert!(
            std::fs::read_to_string(&temp.0)
                .unwrap()
                .contains(r#""amount":100"#)
        );
    }

    #[test]
    fn the_daily_count_sees_submissions_only() {
        let temp = TempLog::new();
        let log = temp.log();
        let today = Utc::now().date_naive();

        log.append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();
        log.append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();
        // Neither of these consumed an order slot.
        log.append(&Record::now(
            "demo",
            Event::Refused {
                symbol: "AAPL".to_owned(),
                instrument_id: 1001,
                action: "OPEN".to_owned(),
                reason: "kill switch".to_owned(),
            },
        ))
        .unwrap();
        log.append(&Record::now(
            "demo",
            Event::Settled {
                reference_id: uuid::Uuid::new_v4(),
                order_id: Some(7),
                status: Some("Filled".to_owned()),
                detail: None,
            },
        ))
        .unwrap();

        assert_eq!(log.submissions_on(today, "demo").unwrap(), 2);
    }

    #[test]
    fn a_retried_order_spends_one_slot_however_many_times_it_was_sent() {
        // Retries reuse their id -- that is the idempotency contract -- so
        // they are one order. Counting records would let a flaky connection
        // exhaust the day's budget without opening a single extra position.
        let temp = TempLog::new();
        let log = temp.log();
        let reference = uuid::Uuid::new_v4();
        for _ in 0..3 {
            log.append(&Record::now("demo", submitted(reference)))
                .unwrap();
        }
        log.append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();

        assert_eq!(log.read().unwrap().len(), 4, "every attempt is recorded");
        assert_eq!(
            log.submissions_on(Utc::now().date_naive(), "demo").unwrap(),
            2,
            "but they are two orders"
        );
    }

    #[test]
    fn the_daily_count_does_not_mix_environments_or_days() {
        let temp = TempLog::new();
        let log = temp.log();
        let today = Utc::now().date_naive();

        log.append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();
        // A real order must never spend a demo budget, or the reverse.
        log.append(&Record::now("real", submitted(uuid::Uuid::new_v4())))
            .unwrap();

        assert_eq!(log.submissions_on(today, "demo").unwrap(), 1);
        assert_eq!(log.submissions_on(today, "real").unwrap(), 1);
        assert_eq!(
            log.submissions_on(today.pred_opt().unwrap(), "demo")
                .unwrap(),
            0,
            "yesterday's budget is not today's"
        );
    }

    #[test]
    fn an_unreadable_line_is_an_error_rather_than_a_silently_lower_count() {
        // The failure a cap must not have: skipping a line it cannot parse
        // would count fewer orders than were actually placed.
        let temp = TempLog::new();
        let log = temp.log();
        log.append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();
        {
            let mut file = fs::OpenOptions::new().append(true).open(&temp.0).unwrap();
            file.write_all(b"{not json}\n").unwrap();
        }

        let error = log.read().unwrap_err();
        assert!(
            matches!(error, AuditError::Corrupt { line: 2, .. }),
            "{error}"
        );
        assert!(
            log.submissions_on(Utc::now().date_naive(), "demo").is_err(),
            "a cap must refuse to guess"
        );
    }

    #[cfg(unix)]
    #[test]
    fn the_log_is_not_world_readable() {
        use std::os::unix::fs::PermissionsExt;
        let temp = TempLog::new();
        temp.log()
            .append(&Record::now("demo", submitted(uuid::Uuid::new_v4())))
            .unwrap();
        let mode = fs::metadata(&temp.0).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600, "it records account activity, not market data");
    }
}
