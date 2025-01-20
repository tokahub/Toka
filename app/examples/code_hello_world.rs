use agents::gpt4free::GPT4FreeAgent;

#[tokio::main]
async fn main() {
    let mut agent = GPT4FreeAgent::new("you are a helpful assistant");

    agent.convert_to_coder();

    let command = "!build:hello_world.rs build me a hello world programm in rust";

    match agent.send_message(command).await {
        Ok(reply) => println!("GPT: {}", reply),
        Err(err) => eprintln!("Error: {}", err),
    }
}
