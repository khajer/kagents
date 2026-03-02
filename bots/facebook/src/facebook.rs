use reqwest::Client;
use serde::Deserialize;

pub struct FacebookClient {
    client: Client,
    access_token: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
pub struct FacebookUser {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookResponse<T> {
    pub data: Option<T>,
    pub error: Option<FacebookError>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub code: u32,
}

impl FacebookClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            access_token: access_token.into(),
            base_url: "https://graph.facebook.com/v18.0".to_string(),
        }
    }

    pub async fn get_user(&self, user_id: &str) -> Result<FacebookUser, Box<dyn std::error::Error>> {
        let url = format!("{}/{}/?access_token={}", self.base_url, user_id, self.access_token);
        let response = self.client.get(&url).send().await?;
        let user: FacebookUser = response.json().await?;
        Ok(user)
    }

    pub async fn get_me(&self) -> Result<FacebookUser, Box<dyn std::error::Error>> {
        self.get_user("me").await
    }
}
