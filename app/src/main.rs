use std::io::{self, Write};
use std::error::Error;
use agents::{base_agent::BaseAgent, gpt4free::GPT4FreeAgent, agent_trait::AgentTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    print_logo();
    let mut agents: Vec<Box<dyn AgentTrait>> = vec![Box::new(GPT4FreeAgent::new("Agent1"))];

    loop {
        print_agents(&agents);
        print_help();

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" {
            break;
        } else if let Some(agent_nr) = input.strip_prefix("chat ") {
            if let Ok(index) = agent_nr.parse::<usize>() {
                if let Some(agent) = agents.get_mut(index) {
                    chat_mode(agent.as_mut()).await?;
                } else {
                    println!("Invalid agent number.");
                }
            }
        } else if let Some(agent_nr) = input.strip_prefix("convert to coder ") {
            if let Ok(index) = agent_nr.parse::<usize>() {
                if let Some(agent) = agents.get_mut(index) {
                    agent.convert_to_coder();
                    println!("Agent {} converted to coder mode!", index);
                } else {
                    println!("Invalid agent number.");
                }
            }
        } else if let Some(filename) = input.strip_prefix("import ") {
            let file_path = format!("export/{}", filename);
            match BaseAgent::import_from_file(&file_path) {
                Ok(agent) => {
                    agents.push(Box::new(agent));
                    println!("Agent imported successfully from {}", file_path);
                }
                Err(e) => println!("Error importing agent: {}", e),
            }
        } else if let Some(agent_nr) = input.strip_prefix("export ") {
            if let Ok(index) = agent_nr.parse::<usize>() {
                if let Some(agent) = agents.get(index) {
                    let file_path = format!("export/agent_{}.agent", agent.get_name());
                    match agent.export_to_file(&file_path) {
                        Ok(_) => println!("Agent {} exported successfully to {}", index, file_path),
                        Err(e) => println!("Error exporting agent: {}", e),
                    }
                } else {
                    println!("Invalid agent number.");
                }
            }
        } else {
            println!("Unknown command.");
        }
    }
    Ok(())
}

fn print_agents(agents: &Vec<Box<dyn AgentTrait>>) {
    println!("\nAvailable Agents:");
    println!("Idx | Type  | Name");
    println!("----------------------------");
    for (i, agent) in agents.iter().enumerate() {
        let agent_type = if agent.is_coder_agent() { "Coder" } else { "Chat" };
        println!("{:3} | {:5} | {}", i, agent_type, agent.get_name());
    }
}

fn print_help() {
    println!("\nCommands:");
    println!("chat <agent nr> - Chat with the specified agent");
    println!("convert to coder <agent nr> - Convert an agent to coder mode");
    println!("import <filename> - import a new agent");
    println!("export <agent nr> - export an agent");
    println!("quit - Exit the program\n");
}

async fn chat_mode(agent: &mut dyn AgentTrait) -> Result<(), Box<dyn Error>> {
    println!("\nEntering chat mode. Type 'quit' to return.");
    loop {
        let mut user_input = String::new();
        print!("> ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut user_input)?;
        let user_input = user_input.trim();

        if user_input == "quit" {
            break;
        }

        match agent.send_message(user_input).await {
            Ok(response) => println!("{}: {}", agent.get_name(), response),
            Err(e) => println!("Error: {}", e),
        }
    }
    Ok(())
}

fn print_logo() {
    let ascii_art = r#"
    ████████╗ ██████╗ ██╗  ██╗ █████╗ 
    ╚══██╔══╝██╔═══██╗██║ ██╔╝██╔══██╗
       ██║   ██║   ██║█████╔╝ ███████║
       ██║   ██║   ██║██╔═██╗ ██╔══██║
       ██║   ╚██████╔╝██║  ██╗██║  ██║
       ╚═╝    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
    "#;
    println!("{}", ascii_art);
}
