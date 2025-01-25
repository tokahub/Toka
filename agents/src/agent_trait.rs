use async_trait::async_trait;
use std::error::Error;
use models::message::Message;

#[async_trait]
pub trait AgentTrait {
    fn get_name(&self) -> &str;
    fn is_coder_agent(&self) -> bool;
    fn convert_to_coder(&mut self);
    fn convert_to_chat(&mut self);
    async fn send_message(&mut self, input: &str) -> Result<String, Box<dyn Error>>;
    fn export_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
    fn import_from_file(file_path: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn set_custom_provider(&mut self, provider: &str);
    fn get_provider(&self) -> Option<&str>;

    fn set_model(&mut self, model: &str);
    fn get_model(&self) -> &str;

    fn set_temperature(&mut self, temperature: f64);
    fn get_temperature(&self) -> Option<f64>;

    fn set_max_tokens(&mut self, max_tokens: u64);
    fn get_max_tokens(&self) -> Option<u64>;

    fn add_system_msg(&mut self, sys_msg: &str);
    fn get_system_messages(&self) -> Vec<&Message>;
}
