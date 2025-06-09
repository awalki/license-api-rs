use reqwest::StatusCode;
use std::collections::HashMap;
use std::error::Error;
use models::{ErrorResponse, LoginRequest, LoginResponse, MeResponse};
use traits::Authenticator;

pub mod hwid;
pub mod models;
pub mod traits;

pub const NOT_LINKED: &str = "not_linked";

pub struct BasicAuthenticator {
    pub base_url: String,
    pub client: reqwest::Client,
}

impl BasicAuthenticator {
    pub fn new(base_url: impl Into<String>) -> Self {
        BasicAuthenticator {
            base_url: base_url.into(),
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl Authenticator for BasicAuthenticator {
    async fn login(
        &self,
        creds: &LoginRequest,
    ) -> Result<LoginResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/license-api-connector/login", self.base_url.trim_end_matches('/'));

        let resp = self.client.post(&url).form(&creds).send().await?;

        match resp.status() {
            StatusCode::OK | StatusCode::CREATED => {
                let lr = resp.json::<LoginResponse>().await?;
                Ok(lr)
            }
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => {
                let err = resp.json::<ErrorResponse>().await?;
                Err(format!("{}", err.detail).into())
            }
            other => Err(format!("unexpected response status: {}", other).into()),
        }
    }

    async fn me(&self, access_token: &str) -> Result<MeResponse, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/users/me", self.base_url.trim_end_matches('/'));

        let resp = self
            .client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => {
                let mr = resp.json::<MeResponse>().await?;
                Ok(mr)
            }
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => {
                let err = resp.json::<ErrorResponse>().await?;
                Err(format!("{}", err.detail).into())
            }
            other => Err(format!("unexpected response status: {}", other).into()),
        }
    }

    async fn link_hwid(
        &self,
        hwid: &str,
        access_token: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/users/hwid", self.base_url.trim_end_matches('/'));

        let mut map = HashMap::new();
        map.insert("value", hwid);

        let resp = self
            .client
            .patch(&url)
            .json(&map)
            .bearer_auth(access_token)
            .send()
            .await?;

        match resp.status() {
            StatusCode::OK => Ok("hwid successfully linked".to_string().into()),
            StatusCode::UNAUTHORIZED | StatusCode::BAD_REQUEST => {
                let err = resp.json::<ErrorResponse>().await?;
                Err(format!("link failed: {}", err.detail).into())
            }
            other => Err(format!("unexpected response status: {}", other).into()),
        }
    }
}
