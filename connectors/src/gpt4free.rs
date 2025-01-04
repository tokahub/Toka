use reqwest::blocking::Client;
use serde_json::json;
use models::{GPTRequest, GPTResponse, Message};
use std::error::Error;

pub struct Agent {
    api_url: String,
    client: Client,
    messages: Vec<Message>,
}

impl Agent {
    // Create a new agent
    pub fn new(api_url: &str, system_content: &str) -> Self {
        let client = Client::new();
        let messages = vec![Message {
            role: "system".to_string(),
            content: system_content.to_string(),
        }];
        Self {
            api_url: api_url.to_string(),
            client,
            messages,
        }
    }

    // Send a message and receive the response
    pub fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        // Build the GPTRequest payload
        let request = GPTRequest {
            model: "gpt-4".to_string(),
            messages: self.messages.clone(),
        };

        // Send the request
        let response = self
            .client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()?;

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