use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "kcli")]
#[command(author = "kagents")]
#[command(version)]
#[command(about = "KAgents CLI ", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Show task or system status")]
    Status {
        #[arg(short, long, help = "Task name or ID")]
        task: Option<String>,
    },
    #[command(about = "list agents work")]
    List {
        #[arg(short, long, help = "Task name or ID")]
        task: Option<String>,
    },
    #[command(about = "add agents")]
    Add {
        #[arg(short, long, help = "add name or ID")]
        task: Option<String>,
    },
    #[command(about = "remove agents")]
    Remove {
        #[arg(short, long, help = "remove agents ")]
        task: Option<String>,
    },
}

const SERVER_URL: &str = "http://127.0.0.1:6411";

#[derive(Deserialize)]
struct ListResponse {
    agents: Vec<Agent>,
}

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub id: i64,
    pub name: String,
    pub token: String,
    pub model: String,
    pub created_at: String,
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {} | Name: {} | Model: {} | Created: {}",
            self.id, self.name, self.model, self.created_at
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Status { task }) => {
            if !check_server_open().await {
                println!("the server doesn't run.");
                return Ok(());
            }
            if let Some(t) = task {
                println!("Status for task: {}", t);
            } else {
                println!("System status: OK");
            }
        }
        Some(Commands::List { task: _ }) => {
            if check_server_open().await {
                send_list().await;
            } else {
                println!("the server doesn't run.");
            }
        }
        Some(Commands::Add { task: _ }) => {
            println!("add agent works ");
        }
        Some(Commands::Remove { task: _ }) => {
            println!("remove agent works ");
        }
        None => {
            println!("Use 'kcli --help' for usage information.");
        }
    }

    Ok(())
}

async fn check_server_open() -> bool {
    match reqwest::get(format!("{}/ping", SERVER_URL)).await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

async fn send_list() {
    match reqwest::get(format!("{}/list", SERVER_URL)).await {
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
