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
