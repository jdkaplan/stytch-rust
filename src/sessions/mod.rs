use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AuthenticateRequest {
    pub session_duration_minutes: Option<u32>,
    pub session_token: Option<String>,
    pub session_jwt: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthenticateResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub session: crate::Session,
    #[serde_as(as = "NoneAsEmptyString")]
    pub session_token: Option<String>,
    pub session_jwt: String,
}

route!(
    http::Method::POST,
    "sessions/authenticate",
    AuthenticateRequest
);

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RevokeRequest {
    pub session_id: Option<String>,
    pub session_token: Option<String>,
    pub session_jwt: Option<String>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RevokeResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,
}

route!(http::Method::POST, "sessions/revoke", RevokeRequest);
