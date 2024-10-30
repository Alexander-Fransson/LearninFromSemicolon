#![allow(dead_code)]

use std::time::Duration;

// in adition to tokio we need reqwest, add by cargo add reqwest
use reqwest::Client;

pub async fn http_example() -> Result<(), Box<reqwest::Error>> {
    let client = Client::builder()
    .timeout(Duration::from_secs(5))
    .build()?;

    println!("Sednding request");

    match client.get("https://httpbin.org/get").send().await {
        Ok(response) => {
            println!("Got response: {:?}", response);
        }
        Err(error) => {
            println!("Got error: {:?}", error);
        }
    }

    Ok(())
}