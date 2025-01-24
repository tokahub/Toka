use crate::base_agent::BaseAgent;
use std::error::Error;
use models::Message;
pub struct GPT4FreeAgent {
    base: BaseAgent,
}

impl GPT4FreeAgent {
    // Create a new GPT4FreeAgent
    pub fn new(name : &str) -> Self {
        // change if gpt4free doesnt run locally for you
        let api_url = "http://localhost:1337/v1/chat/completions";
        let base = BaseAgent::new_with_param(
            name,
            api_url,
            None,
            None,
            Some("gpt-4".to_string()),
            None,
        );
        Self { base }
    }

    pub fn new_with_sys(name : &str, system_content: &str) -> Self {
        // change if gpt4free doesnt run locally for you
        let api_url = "http://localhost:1337/v1/chat/completions";
        let base = BaseAgent::new_with_param(
            name,
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

    pub fn get_custom_provider(&self) -> &str {
        &self.base.model
    }

    // Setter and Getter for temperature
    pub fn set_temperature(&mut self, temperature: f64) {
        self.base.set_temperature(temperature);
    }

    pub fn get_temperature(&self) -> Option<f64> {
        self.base.temperature
    }

    // Setter and Getter for max_tokens
    pub fn set_max_tokens(&mut self, max_tokens: u64) {
        self.base.set_max_tokens(max_tokens);
    }

    pub fn get_max_tokens(&self) -> Option<u64> {
        self.base.max_tokens
    }

    // Add a system message
    pub fn add_system_msg(&mut self, sys_msg: &str) {
        self.base.add_system_msg(sys_msg);
    }

    // Get all system messages
    pub fn get_system_messages(&self) -> Vec<&Message> {
        self.base
            .messages
            .iter()
            .filter(|msg| msg.role == "system")
            .collect()
    }

    // Convert to coder agent
    pub fn convert_to_coder(&mut self) {
        self.base.convert_to_coder();
    }

    // Check if the agent is a coder agent
    pub fn is_coder_agent(&self) -> bool {
        self.base.coder_agent
    }

    // Get the API URL
    pub fn get_api_url(&self) -> &str {
        &self.base.api_url
    }

    // Get the API key
    pub fn get_api_key(&self) -> Option<&String> {
        self.base.api_key.as_ref()
    }

    // Get the model
    pub fn get_model(&self) -> &str {
        &self.base.model
    }

    // Get the provider
    pub fn get_provider(&self) -> Option<&String> {
        self.base.provider.as_ref()
    }

    // Get the name
    pub fn get_name(&self) -> &str  {
        &self.base.name
    }

    pub fn export_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.base.export_to_file(file_path)?;
        Ok(())
    }

    pub fn import_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // reconstruct base
        let base = BaseAgent::import_from_file(file_path)?;
        Ok(Self { base })
    }
}