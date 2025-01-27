use crate::base_agent::BaseAgent;
use std::error::Error;
use models::Message;
use crate::agent_trait::AgentTrait;
use async_trait::async_trait;

pub struct DeepseekAgent {
    base: BaseAgent,
}

impl DeepseekAgent {
    // Create a new DeepseekAgent
    // always need an api key
    pub fn new(name : &str, api_key : &str) -> Self {
        let api_url = "https://api.deepseek.com/chat/completions";
        let base = BaseAgent::new_with_param(
            name,
            api_url,
            Some(api_key.to_string()),
            None,
            Some("deepseek-chat".to_string()),
            None,
        );
        Self { base }
    }

    pub fn new_with_sys(name : &str, system_content: &str, api_key : &str) -> Self {
        let api_url = "https://api.deepseek.com/chat/completions";
        let base = BaseAgent::new_with_param(
            name,
            api_url,
            Some(api_key.to_string()),
            Some(system_content.to_string()),
            Some("deepseek-chat".to_string()),
            None,
        );
        Self { base }
    }
}


#[async_trait]
impl AgentTrait for DeepseekAgent {
    async fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.base.send_message(user_message).await
    }

    fn set_custom_provider(&mut self, provider: &str) {
        self.base.set_model(provider);
    }

    fn get_provider(&self) -> Option<&str> {
        self.base.get_provider()
    }

    fn convert_to_chat(&mut self) {
        self.base.convert_to_chat();
    }

    fn set_model(&mut self, model: &str) {
        self.base.set_model(model);
    }


    fn set_temperature(&mut self, temperature: f64) {
        self.base.set_temperature(temperature);
    }

    fn get_temperature(&self) -> Option<f64> {
        self.base.get_temperature()
    }

    fn set_max_tokens(&mut self, max_tokens: u64) {
        self.base.set_max_tokens(max_tokens);
    }

    fn get_max_tokens(&self) -> Option<u64> {
        self.base.get_max_tokens()
    }

    fn add_system_msg(&mut self, sys_msg: &str) {
        self.base.add_system_msg(sys_msg);
    }

    fn get_system_messages(&self) -> Vec<&Message> {
        self.base.get_system_messages()
    }

    fn convert_to_coder(&mut self) {
        self.base.convert_to_coder();
    }

    fn is_coder_agent(&self) -> bool {
        self.base.is_coder_agent()
    }

    fn convert_to_twitter(&mut self) {
        self.base.convert_to_twitter();
    }

    fn is_twitter_agent(&self) -> bool {
        self.base.is_twitter_agent()
    }

    fn get_model(&self) -> &str {
       self.base.get_model()
    }

    fn get_name(&self) -> &str  {
        &self.base.get_name()
    }

    fn export_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        self.base.export_to_file(file_path)?;
        Ok(())
    }

    fn import_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        // reconstruct base
        let base = BaseAgent::import_from_file(file_path)?;
        Ok(Self { base })
    }
}