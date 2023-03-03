use tide::Request;
use reqwest::Error;
use serde::Deserialize;


#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/status").get(get_status);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn get_status(_req: Request<()>) -> tide::Result {
    get_weather();
    Ok(format!("Ok").into())
}

#[tokio::main]
async fn get_weather() -> Result<(), Error> {
    let request_url = format!("https://archive-api.open-meteo.com/v1/archive?{latitude}&{longitude}&{daily}&{start}&{end}&{timezone}",
                          latitude = "latitude=52.52",
                          longitude = "longitude=13.41",
                          daily = "daily=temperature_2m_min",
                          start = "start_date=2023-01-27",
                          end = "end_date=2023-02-26",
                          timezone = "timezone=GMT-0");

    let response = reqwest::get(&request_url)
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    Ok(())
}
