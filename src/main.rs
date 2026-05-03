// Self-alias so that absolute paths emitted by cargo typify (e.g.
// `::etoro_agent::types::manual::TradeDirection`, generated from x-rust-type
// hints in docs/*-schema.json) resolve from within this binary crate.
extern crate self as etoro_agent;

mod client;
mod types;

use anyhow::Result;

use etoro_agent::client::EtoroClient;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("ETORO_API_KEY")?;
    let user_key = std::env::var("ETORO_USER_KEY")?;

    let client = EtoroClient::new(&api_key, &user_key)?;

    let v = client.watchlists().await?;

    println!(
        "Response:\nStatus: {:?}\nWatchlists: {:#?}",
        v.status, v.watchlists
    );

    Ok(())
}
