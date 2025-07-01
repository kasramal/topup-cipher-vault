use std::time::Duration;

use reqwest::Client;

use crate::application::registry::{Instance, RegisterRequest};
pub async fn register_in_eureka(
    client: &Client,
    instance: Instance,
    app_name: String,
    eureka_base: String,
    attempts: u16
) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/apps/{}", eureka_base, app_name.to_uppercase());
    let req = RegisterRequest { instance };

    for attempt in 1..=attempts {
        match client.post(&url).json(&req).send().await {
            Ok(res) => {
                println!("Register attempt {} => status: {}", attempt, res.status());
                if res.status().is_success() {
                    return Ok(());
                } else {
                    eprintln!("WARNING... Eureka responded with non-success: {}", res.status());
                }
            }
            Err(err) => {
                eprintln!("REGISTERY NOT AVAILABLE! Request error on attempt {}: {}", attempt, err);
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    Err(format!("Eureka registration failed after {attempts} attempts").into())
}

pub async fn heartbeat(client: &Client, instance_id: &String, app_name: &String, eureka_base: &String) -> Result<(), Box<dyn std::error::Error>> {
    let res = client
            .put(format!("{}/apps/{}/{}", eureka_base, app_name.to_uppercase(), instance_id))
            .send()
            .await?;
    println!("Register response: {}", res.status());

    Ok(())
}