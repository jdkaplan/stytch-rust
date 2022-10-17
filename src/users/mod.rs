use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Name {
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "pending")]
    Pending,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CreateRequest {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub name: Option<Name>,
    pub create_user_as_pending: Option<bool>,
    pub attributes: Option<crate::Attributes>,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub user_id: String,
    pub user: crate::User,
    pub email_id: String,
    pub status: Status,
}

route!(http::Method::POST, "users", CreateRequest);
