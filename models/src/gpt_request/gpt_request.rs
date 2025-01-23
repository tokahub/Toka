use serde::Serialize;
use crate::message::Message;

#[derive(Serialize)]
pub struct GPTRequest {
    pub api_key: Option<String>,
    pub messages: Vec<Message>,
    pub model: String,
    pub provider: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
}