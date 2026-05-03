use anyhow::Result;
use reqwest::header;

use crate::types::watchlists::WatchlistsResponse;

const BASE_URL: &str = "https://public-api.etoro.com";

pub struct EtoroClient {
    http: reqwest::Client,
}

impl EtoroClient {
    pub fn new(api_key: &str, user_key: &str) -> Result<Self> {
        let mut headers = header::HeaderMap::new();

        let sensitive = |s: &str| -> Result<header::HeaderValue> {
            let mut hv = header::HeaderValue::from_str(s)?;
            hv.set_sensitive(true);
            Ok(hv)
        };
        headers.insert("x-api-key", sensitive(api_key)?);
        headers.insert("x-user-key", sensitive(user_key)?);

        Ok(Self {
            http: reqwest::Client::builder()
                .default_headers(headers)
                .build()?,
        })
    }

    pub async fn watchlists(&self) -> Result<WatchlistsResponse> {
        let x_request_id = uuid::Uuid::new_v4().to_string();

        let response = self
            .http
            .get(format!("{BASE_URL}/api/v1/watchlists"))
            .header("x-request-id", x_request_id)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(response)
    }
}
