use std::{env, error::Error};

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use serde_discord::register::Command;

pub struct Client {
    app_id: String,
    token: String,
    endpoint: String,
}

impl Client {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        if dotenvy::dotenv().is_err() {
            return Err("Could not load environment variables".into());
        }
        let app_id =
            env::var("DISCORD_APP_ID").expect("Environment variable `DISCORD_APP_ID` is not set");
        let token =
            env::var("DISCORD_TOKEN").expect("Environment variable `DISCORD_TOKEN` is not set");
        let endpoint = "https://discord.com/api/v10/applications".to_string();
        Ok(Self {
            app_id,
            token,
            endpoint,
        })
    }

    fn endpoint_cmds(&self) -> String {
        format!("{}/{}/commands", self.endpoint, self.app_id)
    }

    pub async fn register_commands(&self, cmds: Vec<Command>) {
        let client = reqwest::Client::new();

        let mut headers = HeaderMap::new();
        headers.append(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bot {}", self.token)).unwrap(),
        );

        let response = client
            .put(&self.endpoint_cmds())
            .headers(headers)
            .json(&cmds)
            .send()
            .await
            .unwrap();

        if response.status().is_success() {
            println!("Commands registered")
        } else {
            println!(
                "Error registering commands: {}",
                response.text().await.unwrap()
            );
        }
    }
}
