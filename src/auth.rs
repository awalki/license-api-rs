use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub key: String,
    pub hwid: String,
}

pub struct LicenseAPI {
    url: String,
    client: Client,
}

impl LicenseAPI {
    pub fn new(url: impl Into<String>) -> Self {
        LicenseAPI {
            url: url.into(),
            client: Client::new(),
        }
    }

    pub async fn login(
        &self,
        creds: &LoginRequest,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self
            .client
            .post(&format!("{}/license/auth", self.url.trim_end_matches('/')))
            .json(creds)
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }
}