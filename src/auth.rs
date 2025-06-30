use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::error::Error;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
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

    pub async fn login(&self, creds: &LoginRequest) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let login_resp = self
            .client
            .post(&format!("{}/auth/login", self.url.trim_end_matches('/')))
            .form(creds)
            .send()
            .await?
            .error_for_status()?;

        let access_token: String = login_resp
            .json::<serde_json::Value>()
            .await?
            .get("access_token")
            .and_then(|v| v.as_str().map(String::from))
            .ok_or("missing access_token in response")?;

        let _ = self
            .client
            .patch(&format!("{}/users/hwid", self.url.trim_end_matches('/')))
            .bearer_auth(&access_token)
            .json(&serde_json::json!({ "value": creds.hwid }))
            .send()
            .await?
            .error_for_status()?;

        Ok(true)
    }
}
