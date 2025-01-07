// This example shows how a user can post a tweet
use utils::post_twitter::post_tweet;

#[tokio::main]
async fn main() {
    let consumer_key = "your_api_key";
    let consumer_secret = "your_api_secret";
    // can be Generate -> see setup_oauth.rs
    let access_token = "your_access_token";
    let access_token_secret = "access_token_secret";

    let tweet_text = "allowing agents to post directly on X";

    match post_tweet(consumer_key, consumer_secret, access_token, access_token_secret, tweet_text).await {
        Ok(()) => println!("Successfully posted tweet!"),
        Err(e) => eprintln!("Failed to post tweet: {}", e),
    }
}
