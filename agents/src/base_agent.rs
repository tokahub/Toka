// This is the base class for all different models there is derivitation
// It is recommended to use the correct derived class rather than the BaseAgent
use reqwest::blocking::Client;
use models::{GPTRequest, GPTResponse, Message};
use std::error::Error;

pub struct BaseAgent {
    api_url: String,
    api_key: Option<String>,
    client: Client,
    model: String,
    provider: Option<String>,
    temperature: Option<f64>,
    max_tokens: Option<u64>,
    messages: Vec<Message>,
}

impl BaseAgent {
    // Create a new BaseAgent
    pub fn new(
        api_url: &str,
        api_key: Option<String>,
        system_content: Option<String>,
        model: Option<String>,
        provider: Option<String>,
    ) -> Self {
        let client = Client::new();
        let messages = vec![Message {
            role: "system".to_string(),
            content: system_content.unwrap_or_else(|| "You are a helpful assistant".to_string()),
        }];
        Self {
            api_url: api_url.to_string(),
            api_key,
            client,
            model: model.unwrap_or_else(|| "gpt-3.5-turbo".to_string()),
            provider: provider,
            temperature: None,
            max_tokens: None,
            messages,
        }
    }

    // Set model, temperature, and max tokens
    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    pub fn set_temperature(&mut self, temperature: f64) {
        self.temperature = Some(temperature);
    }

    pub fn set_max_tokens(&mut self, max_tokens: u64) {
        self.max_tokens = Some(max_tokens);
    }

    // Send a message and receive a response
    pub fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        // Build the GPTRequest payload
        let request = GPTRequest {
            model: self.model.clone(),
            api_key: self.provider.clone(),
            provider: self.provider.clone().unwrap_or_else(|| "".to_string()),
            messages: self.messages.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        };

        // Send the request
        let mut request_builder = self
            .client
            .post(&self.api_url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = &self.api_key {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request_builder.json(&request).send()?;

        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }

        // Parse the response
        let body = response.text()?;
        let gpt_response: GPTResponse = serde_json::from_str(&body)?;

        // Extract the reply
        if let Some(choice) = gpt_response.choices.get(0) {
            let reply = choice.message.content.clone();
            self.messages.push(Message {
                role: "assistant".to_string(),
                content: reply.clone(),
            });
            Ok(reply)
        } else {
            Err("No choices in response.".into())
        }
    }
}