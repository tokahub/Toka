use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct TwitterCredentials {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}