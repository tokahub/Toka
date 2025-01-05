use crate::base_agent::BaseAgent;
use std::error::Error;

pub struct GPT4FreeAgent {
    base: BaseAgent,
}

impl GPT4FreeAgent {
    // Create a new GPT4FreeAgent
    pub fn new(system_content: &str, api_key: &str) -> Self {
        let api_url = "https://api.openai.com/v1/chat/completions";
        // Will need an Api key
        let base = BaseAgent::new(
            api_url,
            Some(api_key.to_string()),
            Some(system_content.to_string()),
            Some("gpt-4".to_string()),
            None,
        );
        Self { base }
    }


    pub fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.base.send_message(user_message)
    }

    pub fn set_custom_provider(&mut self, provider: &str) {
        self.base.set_model(provider);
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.base.set_temperature(temperature);
    }

    pub fn set_max_tokens(&mut self, max_tokens: u64) {
        self.base.set_max_tokens(max_tokens);
    }
}