use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::env;

mod command;
use command::{Agent, Cli};
use crate::command::Commands;

mod http_fn;

const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:6411";

fn get_server_url() -> String {
    env::var("SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string())
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
    process_command_line(cli, &server_url).await;

    Ok(())
}

async fn process_command_line(cli: Cli, server_url: &str) {
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
}
