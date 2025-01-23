use serde::Deserialize;
use crate::choice::Choice;
use crate::usage::Usage;

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
