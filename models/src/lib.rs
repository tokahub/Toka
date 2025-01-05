use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct GPTRequest {
    pub api_key: Option<String>,
    pub messages: Vec<Message>,
    pub model: String,
    pub provider: String,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct GPTResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub choices: Vec<Choice>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub index: usize,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}
