mod facebook;

use facebook::FacebookClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let access_token =
        std::env::var("FACEBOOK_ACCESS_TOKEN").expect("FACEBOOK_ACCESS_TOKEN must be set");

    let client = FacebookClient::new(access_token);

    println!("=== Fetching User Info ===");
    match client.get_me().await {
        Ok(user) => println!("User: {:?}\n", user),
        Err(e) => eprintln!("Error fetching user: {}\n", e),
    }

    println!("=== Fetching Feed ===");
    match client.get_my_feed(Some(5)).await {
        Ok(feed) => {
            println!("Found {} posts:", feed.data.len());
            for post in feed.data {
                println!("  - Post ID: {}", post.id);
                if let Some(msg) = post.message {
                    println!("    Message: {}", msg.chars().take(100).collect::<String>());
                }
                if let Some(created) = post.created_time {
                    println!("    Created: {}", created);
                }
            }
            println!();
        }
        Err(e) => eprintln!("Error fetching feed: {}\n", e),
    }

    println!("=== Fetching Photos ===");
    match client.get_my_photos(Some(5)).await {
        Ok(photos) => {
            println!("Found {} photos:", photos.data.len());
            for photo in photos.data {
                println!("  - Photo ID: {}", photo.id);
                if let Some(created) = photo.created_time {
                    println!("    Created: {}", created);
                }
                if let Some(largest) = photo.images.first()
                    && let Some(url) = &largest.source
                {
                    println!("    URL: {}", url);
                }
            }
            println!();
        }
        Err(e) => eprintln!("Error fetching photos: {}\n", e),
    }

    println!("=== Example: Fetch Specific Post ===");
    let post_id = "123456789_987654321";
    match client.get_post(post_id).await {
        Ok(post) => {
            println!("Post ID: {}", post.id);
            if let Some(msg) = post.message {
                println!("Message: {}", msg);
            }
        }
        Err(e) => println!("Note: Could not fetch post (expected): {}", e),
    }

    Ok(())
}
