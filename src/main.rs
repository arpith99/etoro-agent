use anyhow::Result;
use reqwest::header;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("ETORO_API_KEY")?;
    let user_key = std::env::var("ETORO_USER_KEY")?;

    let mut headers = header::HeaderMap::new();

    let sensitive = |s: &str| -> Result<header::HeaderValue> {
        let mut hv = header::HeaderValue::from_str(s)?;
        hv.set_sensitive(true);
        Ok(hv)
    };
    headers.insert("x-api-key", sensitive(&api_key)?);
    headers.insert("x-user-key", sensitive(&user_key)?);

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let x_request_id = uuid::Uuid::new_v4().to_string();

    let response = client
        .get("https://public-api.etoro.com/api/v1/watchlists")
        .header("x-request-id", x_request_id)
        .send()
        .await?
        .error_for_status()?;

    let http_status = response.status();
    let v: Value = response.json().await?;

    println!("HTTP {http_status}");
    println!(
        "Response:\nStatus: {}\nWatchlists: {}",
        v["status"],
        serde_json::to_string_pretty(&v["watchlists"])?
    );

    Ok(())
}
