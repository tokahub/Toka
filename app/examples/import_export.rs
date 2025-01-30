// Example showing how to export and import an agent with verifications

use agents::gpt4free::GPT4FreeAgent;
use std::error::Error;
use agents::agent_trait::AgentTrait;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create a custom GPT4FreeAgent
    let mut agent = GPT4FreeAgent::new("dummy");

    // fill up the paramets
    agent.convert_to_coder();
    agent.set_temperature(0.7);
    agent.set_max_tokens(1000);
    agent.add_system_msg("You are a Rust expert.");

    // Export the agent to a file
    let export_path = "export/dummy.agent";
    agent.export_to_file(export_path)?;
    println!("Agent exported to {}", export_path);

    // Import the agent again from the file
    let imported_agent = GPT4FreeAgent::import_from_file(export_path)?;
    println!("Agent imported from {}", export_path);

    // Assert that all parameters match
    assert_eq!(agent.get_model(), imported_agent.get_model());
    assert_eq!(agent.get_provider(), imported_agent.get_provider());
    assert_eq!(agent.get_temperature(), imported_agent.get_temperature());
    assert_eq!(agent.get_max_tokens(), imported_agent.get_max_tokens());
    assert_eq!(agent.get_system_messages(), imported_agent.get_system_messages());
    assert_eq!(agent.is_coder_agent(), imported_agent.is_coder_agent());


    println!("Exported and imported successfully!");
    Ok(())
}