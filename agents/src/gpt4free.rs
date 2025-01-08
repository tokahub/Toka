use crate::base_agent::BaseAgent;
use std::error::Error;

pub struct GPT4FreeAgent {
    base: BaseAgent,
}

impl GPT4FreeAgent {
    // Create a new GPT4FreeAgent
    pub fn new(system_content: &str) -> Self {
        // change if gpt4free doesnt run locally for you
        let api_url = "http://localhost:1337/v1/chat/completions";
        let base = BaseAgent::new(
            api_url,
            None,
            Some(system_content.to_string()),
            Some("gpt-4".to_string()),
            None,
        );
        Self { base }
    }


    pub async fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.base.send_message(user_message).await
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

    pub fn add_system_msg(&mut self, sys_msg: &str){
        self.base.add_system_msg(sys_msg);
    }

    pub fn convert_to_coder(&mut self){
        self.base.convert_to_coder();
    }
}