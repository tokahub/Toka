use agents::gpt4free::GPT4FreeAgent;

#[tokio::main]
async fn main() {
    let mut agent = GPT4FreeAgent::new();

    agent.convert_to_coder();

    let command = "!build:adder.rs build me a rust program that will add together two user inputs and then print the result";

    match agent.send_message(command).await {
        Ok(reply) => println!("GPT: {}", reply),
        Err(err) => eprintln!("Error: {}", err),
    }
}
