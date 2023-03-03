use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Utc};
use tide::Request;

#[tokio::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/status").get(get_status);
    app.listen("[::]:8080").await?;
    Ok(())
}

async fn get_status(_req: Request<()>) -> tide::Result {
    let neg_temp: bool = get_weather()?;
    if neg_temp {
        Ok(format!("Unfortunately, it is not safe to ride a motorcycle today").into())
    } else {
        Ok(format!("Fortunately, it is safe to ride a motorcycle today").into())
    }
}

#[tokio::main]
async fn get_weather() -> Result<bool, anyhow::Error> {
    let (end, start): (String, String) = get_dates();

    let request_url = format!("https://api.open-meteo.com/v1/forecast?{latitude}&{longitude}&{daily}&start_date={start}&end_date={end}&{timezone}",
                          latitude = "latitude=52.52",
                          longitude = "longitude=13.41",
                          daily = "daily=temperature_2m_min",
                          start = start,
                          end = end,
                          timezone = "timezone=Europe%2FBerlin");

    let response = reqwest::get(&request_url)
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut negative_temperature: bool = false;
    for i in 0..7 {
        let value: f64 =
            serde_json::from_value(response["daily"]["temperature_2m_min"][i].clone())?;
        if value < 1.0 {
            negative_temperature = true;
            break;
        }
    }
    Ok(negative_temperature)
}

fn get_dates() -> (String, String) {
    let now = Utc::now();
    let last_week = now - Duration::days(7);
    (to_iso_format(now), to_iso_format(last_week))
}

fn to_iso_format(date: DateTime<Utc>) -> String {
    let (_is_common_era, year) = date.year_ce();
    return format!("{}-{:02}-{:02}", year, date.month(), date.day());
}
