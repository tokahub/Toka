use agents::gpt4free::GPT4FreeAgent;
use tweety_rs::TweetyClient;
use tokio;
use tokio::io::{self, BufReader, AsyncBufReadExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system_content = "You are an intelligent assistant specialized in writing concise and engaging tweets. Important: use 280 characters or less";
    let mut agent = GPT4FreeAgent::new_with_sys(system_content);

    let client = TweetyClient::new(
        "your_consumer_key",
        "your_consumer_key_secret",
        "your_access_token",
        "your_access_token_secret",
    );

    loop {
        println!("Choose an action: \n1. Post a tweet \n3. Exit");
        let mut choice = String::new();
        let mut reader = BufReader::new(io::stdin());
        reader.read_line(&mut choice).await?;
        let choice = choice.trim();

        match choice {
            "1" => {
                // Post a tweet
                println!("Enter your message: ");
                let mut tweet_content = String::new();
                reader.read_line(&mut tweet_content).await?;  // Read the tweet content
                let tweet_content = tweet_content.trim();

                // Match on the Result returned by send_message
                match agent.send_message(tweet_content).await {
                    Ok(reply) => {
                        println!("Generated tweet: {}", reply);  // Successfully generated a tweet
                        println!("Do you want to tweet this? (y/n): ");

                        let mut confirm = String::new();
                        reader.read_line(&mut confirm).await?;
                        if confirm.trim().eq_ignore_ascii_case("y") {
                            client.post_tweet(&reply, None).await.unwrap();  // Post the tweet
                        } else {
                            println!("Tweet not posted. Returning to menu.");
                        }
                    }
                    Err(err) => {
                        eprintln!("Error generating tweet: {}", err);
                    }
                }
            }
            "3" => {
                println!("Exiting... Goodbye!");
                break;  // Exit the loop and the program
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }

    Ok(())
}