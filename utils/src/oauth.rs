use std::io;
use reqwest::Client;
use reqwest_oauth1::{Secrets, OAuthClientProvider, TokenReaderFuture};
use std::error::Error;

/// Sets up OAuth by interacting with the Twitter API.
///
/// # Arguments
/// * `consumer_key` - Your Twitter API consumer key.
/// * `consumer_secret` - Your Twitter API consumer secret.
///
/// # Returns
/// A tuple `(String, String)` containing the OAuth token and OAuth token secret.
pub async fn setup(consumer_key: &str, consumer_secret: &str) -> Result<(String, String), Box<dyn Error>> {
    // Prepare authorization info
    let secrets = Secrets::new(consumer_key, consumer_secret);

    // Step 1: Acquire request token and token secret
    let endpoint_reqtoken = "https://api.twitter.com/oauth/request_token?oauth_callback=oob&x_auth_access_type=write";
    let client = Client::new();

    let resp = client
        .oauth1(secrets)
        .get(endpoint_reqtoken)
        .query(&[("oauth_callback", "oob")])
        .send()
        .parse_oauth_token()
        .await?;

    // Step 2: Acquire user PIN
    let endpoint_authorize = "https://api.twitter.com/oauth/authorize?oauth_token=";
    println!("Please access the following URL to authorize the app: {}{}", endpoint_authorize, resp.oauth_token);

    println!("Enter the PIN provided by Twitter: ");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");
    let pin = user_input.trim();

    // Step 3: Acquire access token
    let secrets = Secrets::new(consumer_key, consumer_secret)
        .token(resp.oauth_token, resp.oauth_token_secret);
    let endpoint_acctoken = "https://api.twitter.com/oauth/access_token";

    let client = Client::new();
    
    let resp = client
        .oauth1(secrets)
        .get(endpoint_acctoken)
        .query(&[("oauth_verifier", pin)])
        .send()
        .parse_oauth_token()
        .await?;

    // Return the OAuth token and token secret
    Ok((resp.oauth_token, resp.oauth_token_secret))
}
