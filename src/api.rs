use reqwest::{header, Error};
use serde::Deserialize;
use serde_json::Result as JsonResult;

#[derive(Deserialize)]
struct QuoteResponse {
    s: String,
    symbol: Vec<String>,
    last: Vec<f64>,
    // Include other fields if needed
}

pub async fn get_cur_price(ticker: &String) -> Result<f64, String> {
    let url = format!("https://api.marketdata.app/v1/stocks/quotes/{}/", ticker);
    let client = reqwest::Client::new();
    let request = client
        .get(url)
        .header(header::ACCEPT, "application/json")
        .header(
            header::AUTHORIZATION,
            "Token cUdnQ015Z2pDOS05Q2J4aVVOSjRqdS1KUHNYdF9adE1qSU1IQlZmV3BSUT0",
        );
    println!("req sent");
    let response = request.send().await.map_err(|e| e.to_string())?;
    let response_text = response.text().await.map_err(|e| e.to_string())?;

    let quote: QuoteResponse = serde_json::from_str(&response_text).map_err(|e| e.to_string())?;

    // Assuming 'last' is not empty and you want the first value
    quote
        .last
        .get(0)
        .cloned()
        .ok_or("No last price found".to_string())
}
