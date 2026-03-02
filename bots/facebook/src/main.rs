mod facebook;

use facebook::FacebookClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let access_token = std::env::var("FACEBOOK_ACCESS_TOKEN")
        .expect("FACEBOOK_ACCESS_TOKEN must be set");
    
    let client = FacebookClient::new(access_token);
    
    match client.get_me().await {
        Ok(user) => println!("User: {:?}", user),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}
