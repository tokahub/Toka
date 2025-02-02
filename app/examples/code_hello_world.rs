// Example that shows the coding agent build a simple Rust program that prints "Hello, World!" to the console

use agents::gpt4free::GPT4FreeAgent;
use agents::agent_trait::AgentTrait;

#[tokio::main]
async fn main() {
    let mut agent = GPT4FreeAgent::new("builder");

    agent.convert_to_coder();

    let command = "!build:hello_world.rs build me a hello world programm in rust";

    match agent.send_message(command).await {
        Ok(reply) => println!("GPT: {}", reply),
        Err(err) => eprintln!("Error: {}", err),
    }
}
