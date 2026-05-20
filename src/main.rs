// Self-alias so that absolute paths emitted by cargo typify (e.g.
// `::etoro_agent::types::manual::TradeDirection`, generated from x-rust-type
// hints in docs/*-schema.json) resolve from within this binary crate.
extern crate self as etoro_agent;

mod client;
mod types;

use anyhow::Result;
use serde::Serialize;
use std::path::Path;

use etoro_agent::client::EtoroClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("ETORO_API_KEY")?;
    let user_key = std::env::var("ETORO_USER_KEY")?;

    let client = EtoroClient::new(&api_key, &user_key)?;

    let v = client.watchlists().await?;
    dump("watchlists_response.json", &v)?;

    let v = client.portfolio().await?;
    dump("portfolio_response.json", &v)?;

    Ok(())
}

fn dump<T: Serialize>(path: impl AsRef<Path>, value: &T) -> Result<()> {
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(value)?;
    std::fs::write(path, &json)?;
    println!("Wrote {} bytes to {}.", json.len(), path.display());
    Ok(())
}
