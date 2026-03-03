use reqwest::Client;
use serde::Deserialize;

pub struct FacebookClient {
    client: Client,
    access_token: String,
    base_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookUser {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookPost {
    pub id: String,
    pub message: Option<String>,
    pub created_time: Option<String>,
    #[serde(default)]
    pub likes: Option<FacebookSummary>,
    #[serde(default)]
    pub comments: Option<FacebookSummary>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookSummary {
    pub summary: Option<FacebookCount>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookCount {
    pub total_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookFeed {
    pub data: Vec<FacebookPost>,
    pub paging: Option<FacebookPaging>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookPaging {
    pub next: Option<String>,
    pub previous: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookPhoto {
    pub id: String,
    pub created_time: Option<String>,
    #[serde(default)]
    pub images: Vec<FacebookImage>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookImage {
    #[allow(dead_code)]
    pub height: Option<u32>,
    #[allow(dead_code)]
    pub width: Option<u32>,
    pub source: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookPhotos {
    pub data: Vec<FacebookPhoto>,
    pub paging: Option<FacebookPaging>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookPage {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub fan_count: Option<u32>,
    #[serde(default)]
    pub followers_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookInsight {
    pub id: String,
    pub name: String,
    pub period: String,
    pub values: Vec<FacebookInsightValue>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookInsightValue {
    pub value: serde_json::Value,
    pub end_time: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct FacebookInsights {
    pub data: Vec<FacebookInsight>,
}

#[derive(Debug, Deserialize)]
pub struct FacebookError {
    #[allow(dead_code)]
    pub message: String,
    pub error_type: String,
    #[allow(dead_code)]
    pub code: u32,
}

#[derive(Debug, Deserialize)]
pub struct FacebookErrorResponse {
    pub error: FacebookError,
}

impl FacebookClient {
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            access_token: access_token.into(),
            base_url: "https://graph.facebook.com/v18.0".to_string(),
        }
    }

    pub async fn get_user(
        &self,
        user_id: &str,
    ) -> Result<FacebookUser, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}?access_token={}",
            self.base_url, user_id, self.access_token
        );
        let response = self.client.get(&url).send().await?;
        
        let status = response.status();
        let response_text = response.text().await?;
        
        if !status.is_success() {
            return Err(format!(
                "Facebook API returned status {}: {}",
                status, response_text
            ).into());
        }

        let user: FacebookUser = serde_json::from_str(&response_text).map_err(|e| {
            format!(
                "Failed to parse user response: {}. Response was: {}",
                e, response_text
            )
        })?;
        Ok(user)
    }

    pub async fn get_me(&self) -> Result<FacebookUser, Box<dyn std::error::Error>> {
        self.get_user("me").await
    }

    pub async fn get_feed(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> Result<FacebookFeed, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(25);
        let url = format!(
            "{}/{}/feed?access_token={}&limit={}&fields=id,message,created_time,likes.summary(true),comments.summary(true)",
            self.base_url, user_id, self.access_token, limit
        );

        let response = self.client.get(&url).send().await?;
        
        let status = response.status();
        let response_text = response.text().await?;
        
        if !status.is_success() {
            return Err(format!(
                "Facebook API returned status {}: {}",
                status, response_text
            ).into());
        }

        let feed: FacebookFeed = serde_json::from_str(&response_text).map_err(|e| {
            format!(
                "Failed to parse feed response: {}. Response was: {}",
                e, response_text
            )
        })?;
        Ok(feed)
    }

    pub async fn get_my_feed(
        &self,
        limit: Option<u32>,
    ) -> Result<FacebookFeed, Box<dyn std::error::Error>> {
        self.get_feed("me", limit).await
    }

    pub async fn get_photos(
        &self,
        user_id: &str,
        limit: Option<u32>,
    ) -> Result<FacebookPhotos, Box<dyn std::error::Error>> {
        let limit = limit.unwrap_or(25);
        let url = format!(
            "{}/{}/photos?access_token={}&limit={}&fields=id,created_time,images",
            self.base_url, user_id, self.access_token, limit
        );

        let response = self.client.get(&url).send().await?;
        
        let status = response.status();
        let response_text = response.text().await?;
        
        if !status.is_success() {
            return Err(format!(
                "Facebook API returned status {}: {}",
                status, response_text
            ).into());
        }

        let photos: FacebookPhotos = serde_json::from_str(&response_text).map_err(|e| {
            format!(
                "Failed to parse photos response: {}. Response was: {}",
                e, response_text
            )
        })?;
        Ok(photos)
    }

    pub async fn get_my_photos(
        &self,
        limit: Option<u32>,
    ) -> Result<FacebookPhotos, Box<dyn std::error::Error>> {
        self.get_photos("me", limit).await
    }

    #[allow(dead_code)]
    pub async fn get_page(
        &self,
        page_id: &str,
    ) -> Result<FacebookPage, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}?access_token={}&fields=id,name,category,fan_count,followers_count",
            self.base_url, page_id, self.access_token
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error: FacebookErrorResponse = response.json().await?;
            return Err(format!(
                "Facebook API Error: {} - {}",
                error.error.error_type, error.error.message
            )
            .into());
        }

        let page: FacebookPage = response.json().await?;
        Ok(page)
    }

    #[allow(dead_code)]
    pub async fn get_insights(
        &self,
        page_id: &str,
        metrics: &[&str],
    ) -> Result<FacebookInsights, Box<dyn std::error::Error>> {
        let metrics_str = metrics.join(",");
        let url = format!(
            "{}/{}/insights?access_token={}&metric={}",
            self.base_url, page_id, self.access_token, metrics_str
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error: FacebookErrorResponse = response.json().await?;
            return Err(format!(
                "Facebook API Error: {} - {}",
                error.error.error_type, error.error.message
            )
            .into());
        }

        let insights: FacebookInsights = response.json().await?;
        Ok(insights)
    }

    pub async fn get_post(
        &self,
        post_id: &str,
    ) -> Result<FacebookPost, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/{}?access_token={}&fields=id,message,created_time,likes.summary(true),comments.summary(true)",
            self.base_url, post_id, self.access_token
        );

        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            let error: FacebookErrorResponse = response.json().await?;
            return Err(format!(
                "Facebook API Error: {} - {}",
                error.error.error_type, error.error.message
            )
            .into());
        }

        let post: FacebookPost = response.json().await?;
        Ok(post)
    }
}
