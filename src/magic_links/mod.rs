use serde::{Deserialize, Serialize};

pub mod email;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    pub token: String,
    pub session_duration_minutes: Option<u32>,
    pub session_token: Option<String>,
    pub session_jwt: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub user_id: String,
    pub user: crate::User,
    pub session: Option<crate::Session>,
    pub session_token: String,
    pub session_jwt: String,
}

route!(
    http::Method::POST,
    "magic_links/authenticate",
    AuthenticateRequest
);
