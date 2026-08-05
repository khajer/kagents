use serde::{Deserialize, Serialize};
use crate::command::Agent;

#[derive(Deserialize)]
pub struct ListResponse {
    pub agents: Vec<Agent>,
}

#[derive(Serialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub token: String,
    pub model: String,
    pub brand: String,
}

#[derive(Deserialize)]
pub struct CreateAgentResponse {
    pub id: i64,
    pub message: String,
}

#[derive(Serialize)]
pub struct RemoveAgentRequest {
    pub id: i64,
}

#[derive(Deserialize)]
pub struct RemoveAgentResponse {
    pub message: String,
}

pub async fn check_server_open(server_url: &str) -> bool {
    match reqwest::get(format!("{}/ping", server_url)).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

pub async fn send_list(server_url: &str) {
    match reqwest::get(format!("{}/list", server_url)).await {
        Ok(response) => match response.json::<ListResponse>().await {
            Ok(list_response) => {
                if list_response.agents.is_empty() {
                    println!("No agents found.");
                } else {
                    for agent in list_response.agents {
                        println!("{}", agent);
                    }
                }
            }
            Err(e) => eprintln!("Failed to parse response: {}", e),
        },
        Err(e) => eprintln!("Failed to connect to server: {}", e),
    }
}

pub async fn add_agent_request(name: &str, token: &str, model: &str, brand: &str, server_url: &str) -> Result<CreateAgentResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request_body = CreateAgentRequest {
        name: name.to_string(),
        token: token.to_string(),
        model: model.to_string(),
        brand: brand.to_string(),
    };

    let response = client
        .post(format!("{}/add", server_url))
        .json(&request_body)
        .send()
        .await?;

    let create_response = response.json::<CreateAgentResponse>().await?;
    Ok(create_response)
}

pub async fn remove_agent_request(id: i64, server_url: &str) -> Result<RemoveAgentResponse, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request_body = RemoveAgentRequest { id };

    let response = client
        .delete(format!("{}/remove", server_url))
        .json(&request_body)
        .send()
        .await?;

    let remove_response = response.json::<RemoveAgentResponse>().await?;
    Ok(remove_response)
}

#[derive(Deserialize)]
pub struct VersionResponse {
    pub version: String,
}

pub async fn get_compatible_version(server_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(format!("{}/compatible_client_version", server_url)).await?;
    let version_response = response.json::<VersionResponse>().await?;
    Ok(version_response.version)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{routing::{delete, get, post}, Router};

    // ponytail: spins a real (ephemeral-port) axum server instead of mocking reqwest,
    // reuses the axum dep already in Cargo.toml rather than pulling in a mock-http crate.
    // Raw JSON strings (not Json<T>) since these response structs only derive Deserialize;
    // reqwest's .json() parses body bytes regardless of content-type.
    async fn spawn_test_server() -> String {
        let app = Router::new()
            .route("/ping", get(|| async { "" }))
            .route("/compatible_client_version", get(|| async {
                r#"{"version":"9.9.9"}"#
            }))
            .route("/add", post(|| async {
                r#"{"id":42,"message":"created"}"#
            }))
            .route("/remove", delete(|| async {
                r#"{"message":"removed"}"#
            }));

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        format!("http://{}", addr)
    }

    #[tokio::test]
    async fn ping_true_when_server_up() {
        let url = spawn_test_server().await;
        assert!(check_server_open(&url).await);
    }

    #[tokio::test]
    async fn ping_false_when_server_down() {
        assert!(!check_server_open("http://127.0.0.1:1").await);
    }

    #[tokio::test]
    async fn add_agent_returns_response() {
        let url = spawn_test_server().await;
        let resp = add_agent_request("bot", "tok", "model", "brand", &url).await.unwrap();
        assert_eq!(resp.id, 42);
        assert_eq!(resp.message, "created");
    }

    #[tokio::test]
    async fn remove_agent_returns_response() {
        let url = spawn_test_server().await;
        let resp = remove_agent_request(1, &url).await.unwrap();
        assert_eq!(resp.message, "removed");
    }

    #[tokio::test]
    async fn compatible_version_parses() {
        let url = spawn_test_server().await;
        let version = get_compatible_version(&url).await.unwrap();
        assert_eq!(version, "9.9.9");
    }
}
