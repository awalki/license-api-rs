use futures_util::SinkExt;
use http::Uri;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process;
use std::str::FromStr;
use tokio::time::Duration;
use tokio::time::sleep;
use tokio_websockets::{ClientBuilder, Message};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub hwid: String,
}

pub struct LicenseAPI {
    url: String,
    client: Client,
    token: String,
}

impl LicenseAPI {
    pub fn new(url: impl Into<String>) -> Self {
        LicenseAPI {
            url: url.into(),
            client: Client::new(),
            token: String::new(),
        }
    }

    pub async fn login(
        &mut self,
        creds: &LoginRequest,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let login_resp = self
            .client
            .post(&format!("{}/auth/login", self.url.trim_end_matches('/')))
            .json(creds)
            .send()
            .await?
            .error_for_status()?;

        self.token = login_resp
            .json::<serde_json::Value>()
            .await?
            .get("access_token")
            .and_then(|v| v.as_str().map(String::from))
            .ok_or("missing access_token in response")?;

        Ok(true)
    }

    pub async fn connect_to_websocket(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        if self.token.is_empty() {
            return Err("You must login before connecting to the WebSocket.".into());
        }

        let base_ws = self
            .url
            .trim_end_matches('/')
            .replace("http://", "ws://")
            .replace("https://", "wss://");
        let url_str = format!("{}/ws/notify?token={}", base_ws, self.token);
        let uri = Uri::from_str(&url_str)?;

        let (mut client, _) = ClientBuilder::from_uri(uri).connect().await?;

        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(30)).await;

                if let Err(_e) = client.send(Message::ping(Vec::new())).await {
                    eprintln!("Check your internet connection: Exiting...");
                    process::exit(1);
                }
            }
        });

        Ok(())
    }
}
