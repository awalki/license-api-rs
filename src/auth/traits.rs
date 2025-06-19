use std::error::Error;
use crate::auth::models::{LoginRequest, LoginResponse, MeResponse};

#[async_trait::async_trait]
pub trait Authenticator {
    async fn login(
        &self,
        creds: &LoginRequest,
        hwid: &str
    ) -> Result<LoginResponse, Box<dyn Error + Send + Sync>>;

    async fn me(&self, access_token: &str) -> Result<MeResponse, Box<dyn Error + Send + Sync>>;

    async fn link_hwid(
        &self,
        hwid: &str,
        access_token: &str,
    ) -> Result<String, Box<dyn Error + Send + Sync>>;
}
