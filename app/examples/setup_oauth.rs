// Example of how a user can get an acces token and access token secret for twitters OAuth1

use utils::oauth;

#[tokio::main]
async fn main() {
    let consumer_key = "your_api_key";
    let consumer_secret = "your_api_secret";

    match oauth::setup(consumer_key, consumer_secret).await {
        Ok((token, secret)) => {
            println!("Access Token: {}", token);
            println!("Access Token Secret: {}", secret);
        }
        Err(e) => {
            eprintln!("Error setting up OAuth: {}", e);
        }
    }
}