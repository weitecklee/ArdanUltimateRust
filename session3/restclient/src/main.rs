#![allow(dead_code)]

const URL: &str = "https://api.open-meteo.com/v1/forecast?latitude=28.3756&longitude=-81.5509&current_weather=true&temperature_unit=fahrenheit";

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Weather {
    latitude: f64,
    longitude: f64,
    current_weather: CurrentWeather,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let response = reqwest::get(URL).await?;
    // println!("{:?}", response.text().await);
    let weather: Weather = response.json().await?;
    // let weather: serde_json::Value = response.json().await?;
    println!("{weather:#?}");
    Ok(())
}
