use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct MeResponse {
    pub telegram_id: String,
    pub username: String,
    pub is_banned: bool,
    pub is_admin: bool,
    pub hwid: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Deserialize, Debug)]
pub struct ErrorResponse {
    pub detail: String,
}
