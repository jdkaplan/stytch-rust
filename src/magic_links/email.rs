use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SendRequest {
    pub email: String,
    pub login_magic_link_url: Option<String>,
    pub signup_magic_link_url: Option<String>,
    pub login_expiration_minutes: Option<u32>,
    pub signup_expiration_minutes: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub user_id: String,
    pub email_id: String,
}

route!(http::Method::POST, "magic_links/email/send", SendRequest);
