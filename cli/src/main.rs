use agents::gpt4free::GPT4FreeAgent;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter system content: ");
    io::stdout().flush()?;
    let mut system_content = String::new();
    io::stdin().read_line(&mut system_content)?;
    let system_content = system_content.trim();

    let mut agent = GPT4FreeAgent::new(system_content);

    loop {
        print!("USER: ");
        io::stdout().flush()?;
        let mut user_message = String::new();
        io::stdin().read_line(&mut user_message)?;
        let user_message = user_message.trim();

        if user_message == "exit" {
            break;
        }

        match agent.send_message(user_message) {
            Ok(reply) => println!("GPT: {}", reply),
            Err(err) => eprintln!("Error: {}", err),
        }
    }

    Ok(())
}