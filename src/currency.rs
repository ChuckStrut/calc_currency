use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ExchangeResponse {
    pub rates: HashMap<String, f64>,
    pub base: String,
    pub date: String,
}

pub async fn fetch_latest_rates() -> Result<ExchangeResponse, reqwest::Error> {
    let url = "https://api.exchangerate.host/latest";
    let res = reqwest::get(url).await?;
    res.json::<ExchangeResponse>().await
}
