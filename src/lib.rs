use serde::{Deserialize, Serialize};
use url::Url;

const LIVE_URL: &str = "https://api.stytch.com/v1/";
const TEST_URL: &str = "https://test.stytch.com/v1/";

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "String")]
pub enum Env {
    Live,
    Test,
    Dev(String),
}

impl From<String> for Env {
    fn from(s: String) -> Self {
        return match s.as_str() {
            "live" => Env::Live,
            "Live" => Env::Live,
            "LIVE" => Env::Live,
            "test" => Env::Test,
            "Test" => Env::Test,
            "TEST" => Env::Test,
            _ => Env::Dev(s),
        };
    }
}

impl Env {
    pub fn base_url(self) -> std::result::Result<Url, url::ParseError> {
        match self {
            Env::Live => Url::parse(LIVE_URL),
            Env::Test => Url::parse(TEST_URL),
            Env::Dev(url) => {
                // The trailing slash is significant in the base URL. Without it, any later joins
                // would drop the last path segment.
                if url.ends_with('/') {
                    Url::parse(&url)
                } else {
                    Url::parse(&(url + "/"))
                }
            }
        }
    }
}

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0:?}")]
    Response(ErrorResponse),

    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error(transparent)]
    InvalidUrl(#[from] url::ParseError),

    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Request<Body: Serialize + Send> {
    pub method: http::Method,
    pub path: String,
    pub body: Body,
}

#[derive(Clone)]
pub struct Config {
    pub base_url: url::Url,
    pub project_id: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(with = "http_serde::status_code")]
    pub status_code: http::StatusCode,
    pub request_id: String,

    pub error_type: String,
    pub error_message: String,
    pub error_url: String,
}

type Timestamp = chrono::DateTime<chrono::Utc>;

// TODO: User
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,

    pub authentication_factors: Vec<AuthenticationFactor>,

    pub started_at: Timestamp,
    pub expires_at: Timestamp,
    pub last_accessed_at: Timestamp,

    pub attributes: Attributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AuthenticationFactor {
    pub delivery_method: String,
    pub r#type: String,
    pub last_authenticated_at: Timestamp,

    #[serde(flatten)]
    pub factor: Factor,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Factor {
    #[serde(rename = "email_factor")]
    Email {
        #[serde(rename = "email_id")]
        id: String,
        #[serde(rename = "email_address")]
        address: String,
    },
    #[serde(rename = "phone_number_factor")]
    PhoneNumber {
        #[serde(rename = "phone_id")]
        id: String,
        #[serde(rename = "phone_number")]
        number: String,
    },
    // TODO: Fill in other factor variants
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct Attributes {
    pub ip_address: String,
    pub user_agent: String,
}

macro_rules! route {
    ( $method:expr, $path:literal, $Req:ty) => {
        impl $Req {
            pub fn build(self) -> crate::Request<Self> {
                crate::Request {
                    method: $method,
                    path: $path.into(),
                    body: self,
                }
            }
        }
    };
}

#[cfg(feature = "reqwest")]
pub mod reqwest;

pub mod magic_links;
pub mod sessions;

#[cfg(test)]
mod tests {
    use super::*;

    fn timestamp(s: &str) -> anyhow::Result<Timestamp> {
        Ok(chrono::DateTime::parse_from_rfc3339(s)?.with_timezone(&chrono::Utc))
    }

    #[test]
    fn deserialize_session() -> anyhow::Result<()> {
        let data = r#"
{
  "attributes": {
    "ip_address": "203.0.113.1",
    "user_agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36"
  },
  "authentication_factors": [
    {
      "delivery_method": "email",
      "email_factor": {
        "email_address": "sandbox@stytch.com",
        "email_id": "email-test-81bf03a8-86e1-4d95-bd44-bb3495224953"
      },
      "last_authenticated_at": "2021-08-09T07:41:52Z",
      "type": "magic_link"
    }
  ],
  "expires_at": "2021-08-10T07:41:52Z",
  "last_accessed_at": "2021-08-09T07:41:52Z",
  "session_id": "session-test-fe6c042b-6286-479f-8a4f-b046a6c46509",
  "started_at": "2021-08-09T07:41:52Z",
  "user_id": "user-test-16d9ba61-97a1-4ba4-9720-b03761dc50c6"
}
        "#;
        let session: Session = serde_json::from_str(data)?;

        let expected = Session {
            attributes: Attributes{
                ip_address: "203.0.113.1".to_string(),
                user_agent: "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36".to_string()
            },
            authentication_factors: vec![
                AuthenticationFactor{
                    delivery_method: "email".to_string(),
                    factor: Factor::Email{
                        address: "sandbox@stytch.com".to_string(),
                        id: "email-test-81bf03a8-86e1-4d95-bd44-bb3495224953".to_string()
                    },
                    last_authenticated_at: timestamp("2021-08-09T07:41:52Z")?,
                    r#type: "magic_link".to_string()
                }
            ],
            expires_at: timestamp("2021-08-10T07:41:52Z")?,
            last_accessed_at: timestamp("2021-08-09T07:41:52Z")?,
            session_id: "session-test-fe6c042b-6286-479f-8a4f-b046a6c46509".to_string(),
            started_at: timestamp("2021-08-09T07:41:52Z")?,
            user_id: "user-test-16d9ba61-97a1-4ba4-9720-b03761dc50c6".to_string(),
};
        assert_eq!(session, expected);
        Ok(())
    }
}
