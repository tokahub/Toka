use oauth1::Token;
use reqwest::Client;
use serde_json::json;

pub async fn post_tweet(
    consumer_key: &str,
    consumer_secret: &str,
    access_token: &str,
    access_token_secret: &str,
    tweet_text: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // OAuth token setup
    let consumer = Token::new(consumer_key, consumer_secret);
    let access = Token::new(access_token, access_token_secret);

    // Request URL
    let url = "https://api.twitter.com/2/tweets";

    // Request body
    let body = json!({
        "text": tweet_text
    });

    // Generate OAuth header
    let auth_header = oauth1::authorize(
        "POST",       // HTTP method
        url,          // API endpoint
        &consumer,    // Consumer token
        Some(&access), // Access token
        None
    );

    // Create the HTTP client
    let client = Client::new();

    // Send the POST request
    let response = client
        .post(url)
        .header("Authorization", auth_header) // Include the OAuth header
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    // Handle the response
    if response.status().is_success() {
        Ok(())
    } else {
        let error_text = response.text().await?;
        Err(format!("{}", error_text).into())
    }
}
