use tokio_websockets::{ClientBuilder, Error, Message, ServerBuilder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use std::error::Error;
use tokio::time::Duration;
use tokio::time::sleep;

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
        let url = format!("{}/ws/notify?token={}", base_ws, self.token);

        let (mut client, _) = ClientBuilder::from_uri(url).connect().await?;

        loop {
            sleep(Duration::from_secs(30)).await;

            if let Err(e) = client.send(Message::ping(Vec::new())).await {
                eprintln!("Ping failed: {}. Exiting.", e);
                process::exit(1);
            }
        }
    }
}
