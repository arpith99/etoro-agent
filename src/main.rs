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

    Ok(())
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
