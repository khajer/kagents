use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

mod command;
use command::{Agent, Cli};
use crate::command::Commands;

mod http_fn;

fn get_server_url() -> String {
    env::var("SERVER_URL").unwrap_or_else(|_| "http://127.0.0.1:6411".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();


    tracing_subscriber::fmt::init();

    let server_url = get_server_url();
    let cli = Cli::parse();

    if !http_fn::check_server_open(&server_url).await {
        println!("the server doesn't run.");
        return Ok(())
    }

    match cli.command {
        Some(Commands::Status { task }) => {
            if let Some(t) = task {
                println!("Status for task: {}", t);
            } else {
                println!("System status: OK");
            }
        }
        Some(Commands::List { task: _ }) => {
            http_fn::send_list(&server_url).await;
        }
        Some(Commands::Add { name, token, model }) => {
            match http_fn::add_agent_request(&name, &token, &model, &server_url).await {
                Ok(response) => {
                    println!("{} (ID: {})", response.message, response.id);
                }
                Err(e) => {
                    eprintln!("Failed to create agent: {}", e);
                }
            }
        }
        Some(Commands::Remove { id }) => {
            match http_fn::remove_agent_request(id, &server_url).await {
                Ok(response) => {
                    println!("{}", response.message);
                }
                Err(e) => {
                    eprintln!("Failed to remove agent: {}", e);
                }
            }
        }
        None => {
            println!("Use 'kcli --help' for usage information.");
        }
    }
    Ok(())
}


// #[derive(Deserialize)]
// struct ListResponse {
//     agents: Vec<Agent>,
// }

// #[derive(Serialize)]
// struct CreateAgentRequest {
//     name: String,
//     token: String,
//     model: String,
// }

// #[derive(Deserialize)]
// struct CreateAgentResponse {
//     id: i64,
//     message: String,
// }

// #[derive(Serialize)]
// struct RemoveAgentRequest {
//     id: i64,
// }

// #[derive(Deserialize)]
// struct RemoveAgentResponse {
//     message: String,
// }
// async fn send_list() {
//     let server_url = get_server_url();
//     match reqwest::get(format!("{}/list", server_url)).await {
//         Ok(response) => match response.json::<ListResponse>().await {
//             Ok(list_response) => {
//                 if list_response.agents.is_empty() {
//                     println!("No agents found.");
//                 } else {
//                     for agent in list_response.agents {
//                         println!("{}", agent);
//                     }
//                 }
//             }
//             Err(e) => eprintln!("Failed to parse response: {}", e),
//         },
//         Err(e) => eprintln!("Failed to connect to server: {}", e),
//     }
// }

// async fn add_agent_request(name: &str, token: &str, model: &str) -> Result<CreateAgentResponse, Box<dyn std::error::Error>> {
//     let server_url = get_server_url();
//     let client = reqwest::Client::new();
//     let request_body = CreateAgentRequest {
//         name: name.to_string(),
//         token: token.to_string(),
//         model: model.to_string(),
//     };

//     let response = client
//         .post(format!("{}/add", server_url))
//         .json(&request_body)
//         .send()
//         .await?;

//     let create_response = response.json::<CreateAgentResponse>().await?;
//     Ok(create_response)
// }

// async fn remove_agent_request(id: i64) -> Result<RemoveAgentResponse, Box<dyn std::error::Error>> {
//     let server_url = get_server_url();
//     let client = reqwest::Client::new();
//     let request_body = RemoveAgentRequest { id };

//     let response = client
//         .delete(format!("{}/remove", server_url))
//         .json(&request_body)
//         .send()
//         .await?;

//     let remove_response = response.json::<RemoveAgentResponse>().await?;
//     Ok(remove_response)
// }
