use tide::Request;
use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct Weather {
    latitude: String,
    longitude: String
}

#[derive(Deserialize, Debug)]
struct JSONResponse {
    json: HashMap<String, String>,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/status").get(get_status);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_status(_req: Request<()>) -> tide::Result {
    let resp: bool = get_weather()?;
    Ok(format!("{resp}", resp = resp).into())
}

#[tokio::main]
async fn get_weather() -> Result<bool, anyhow::Error> {
    let request_url = format!("https://archive-api.open-meteo.com/v1/archive?{latitude}&{longitude}&{daily}&{start}&{end}&{timezone}",
                          latitude = "latitude=52.52",
                          longitude = "longitude=13.41",
                          daily = "daily=temperature_2m_min",
                          start = "start_date=2023-01-01",
                          end = "end_date=2023-01-08",
                          timezone = "timezone=GMT-0");

    let response = reqwest::get(&request_url).await?.json::<serde_json::Value>().await?;

    let mut negative_temperature: bool = false;
    for i in 0..7 {
        let value: f64 = serde_json::from_value(response["daily"]["temperature_2m_min"][i].clone())?;
        if value < 1.0 {
            negative_temperature = true;
            break;
        }
    }

    Ok(negative_temperature)
}
