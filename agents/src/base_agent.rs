// This is the base class for all different models there is derivitation
// It is recommended to use the correct derived class rather than the BaseAgent
use reqwest::Client;
use models::{GPTRequest, GPTResponse, Message};
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};
use base64::{encode, decode};

#[derive(Serialize, Deserialize)]
pub struct BaseAgent {
    pub api_url: String,
    pub api_key: Option<String>,
    #[serde(skip_serializing, skip_deserializing)] 
    pub client: Client,
    pub model: String,
    pub provider: Option<String>,
    pub temperature: Option<f64>,
    pub max_tokens: Option<u64>,
    pub messages: Vec<Message>,
    pub coder_agent: bool,
}

impl BaseAgent {
    // Create a new BaseAgent
    pub fn new_with_param(
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
            coder_agent: false,
        }
    }

    pub fn new(api_url: &str) -> Self{
        Self::new_with_param(api_url, None, None, None, None)
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

    pub fn add_system_msg(&mut self, sys_msg: &str){
        self.messages.push(Message {
            role: "system".to_string(),
            content: sys_msg.to_string(),
        });
    }


    // Build the GPTRequest payload
    fn build_gpt_request(&self) -> GPTRequest {
        GPTRequest {
            model: self.model.clone(),
            api_key: self.api_key.clone(),
            provider: self.provider.clone().unwrap_or_else(|| "".to_string()),
            messages: self.messages.clone(),
            temperature: self.temperature,
            max_tokens: self.max_tokens,
        }
    }

    // Send the GPT request
    async fn send_gpt_request(&self, request: &GPTRequest) -> Result<reqwest::Response, Box<dyn Error>> {
        let mut request_builder = self
            .client
            .post(&self.api_url)
            .header("Content-Type", "application/json");

        if let Some(api_key) = &request.api_key {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", api_key));
        }

        let response = request_builder.json(&request).send().await?;

        if !response.status().is_success() {
            return Err(format!("HTTP Error: {}", response.status()).into());
        }

        Ok(response)
    }

    // Extract the reply from the GPT response
    async fn extract_reply_from_response(&self, response: reqwest::Response) -> Result<String, Box<dyn Error>> {
        let body = response.text().await?;
        let gpt_response: GPTResponse = serde_json::from_str(&body)?;

        if let Some(choice) = gpt_response.choices.get(0) {
            Ok(choice.message.content.clone())
        } else {
            Err("No response.".into())
        }
    }

    // Send a message and receive a response
    pub async fn send_message(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        if self.coder_agent {
            self.handle_coder_agent(user_message).await
        } else {
            self.handle_normal_conversation(user_message).await
        }
    }

    // Handle normal conversation
    async fn handle_normal_conversation(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        self.messages.push(Message {
            role: "user".to_string(),
            content: user_message.to_string(),
        });

        let request = self.build_gpt_request();
        let response = self.send_gpt_request(&request).await?;
        let reply = self.extract_reply_from_response(response).await?;

        self.messages.push(Message {
            role: "assistant".to_string(),
            content: reply.clone(),
        });

        Ok(reply)
    }
    
    async fn handle_coder_agent(&mut self, user_message: &str) -> Result<String, Box<dyn Error>> {
        let build_mode = user_message.starts_with("!build");
       
        /*
        if build_mode
        {
            println!("builder mode active");
        }
        else {
            println!("builder mode not active")
        }
        */

        if !build_mode {
            return self.handle_normal_conversation(user_message).await;
        }

        let mut filename = String::new();
        if user_message.starts_with("!build:"){ // get name from build argument
            let parts: Vec<&str> = user_message.splitn(2, ':').collect();
            filename = parts[1].split_whitespace().next().unwrap_or("").to_string();
        }
        else { //ask for a filename
            println!("Enter a filename to save the code (or leave empty to cancel):");
            io::stdin().read_line(&mut filename)?;
            filename = filename.trim().to_string();
        } 

        if filename.is_empty() {
            println!("Normal response:");
            return self.handle_normal_conversation(user_message).await;
        }

        // Get the code from the normal conversation
        let code = self.handle_normal_conversation(user_message).await?;

        // Save the code to a file
        let file_path = Path::new("output").join(filename);

        // Create the output directory if it doesn't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Write the code to the file
        fs::write(&file_path, code)?;

        Ok(format!("Code saved to {}", file_path.display()))
    }

    // Changes the current agent to a coder_agent
    pub fn convert_to_coder(&mut self)
    {
        if self.coder_agent == true
        {
            println!("Already a coder agent");
            return
        }

        // TODO change so it only clears system messages 
        self.messages.clear();

        let system_message = "You are a code generator. Your task is to generate working code based on the user's input.
            Important: - Only generate code and comments. 
            - Do not include anything else, such as code block markers (```) or language labels. 
            - The code must be usable without removing anything.
            - Do not include anythin but the code part itself";
        self.messages.push(Message{
            role: "system".to_string(),
            content: system_message.to_string(),
        });

        self.coder_agent = true;
    }

    // Export the agent
    pub fn export_to_file(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string(self)?;
        let encoded = encode(json);

        let mut file = File::create(file_path)?;
        file.write_all(encoded.as_bytes())?;

        Ok(())
    }

    // Import the agent
    pub fn import_from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let encoded = fs::read_to_string(file_path)?;
        let decoded = decode(encoded)?;
        let json = String::from_utf8(decoded)?;
        let mut agent: BaseAgent = serde_json::from_str(&json)?;
        
        agent.client = Client::new();

        Ok(agent)
    }
}