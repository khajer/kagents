use clap::Parser;
use dotenv::dotenv;
use std::env;

mod command;
use command::Cli;
use crate::command::Commands;

mod http_fn;

const DEFAULT_SERVER_URL: &str = "http://127.0.0.1:6411";
const CLIENT_VERSION: &str = "1.0.0";
const ERR_MSG_SERVER_NOT_RUNNING: &str = "the server doesn't run.";
const MSG_FAIL_CREATE: &str = "Failed to create agent";
const MSG_FAIL_REMOVE: &str = "Failed to remove agent";

fn get_server_url() -> String {
    env::var("SERVER_URL").unwrap_or_else(|_| DEFAULT_SERVER_URL.to_string())
}

/// Returns true if `a` is semantically lower than `b`.
/// Compares dotted version strings (e.g. "1.0.0") part by part.
fn is_version_lower(a: &str, b: &str) -> bool {
    let parse = |v: &str| -> Vec<u32> {
        v.split('.')
            .map(|p| p.parse::<u32>().unwrap_or(0))
            .collect()
    };
    let va = parse(a);
    let vb = parse(b);
    for (x, y) in va.iter().zip(vb.iter()) {
        if x < y { return true; }
        if x > y { return false; }
    }
    false
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let server_url = get_server_url();
    let cli = Cli::parse();

    if !http_fn::check_server_open(&server_url).await {
        println!("{ERR_MSG_SERVER_NOT_RUNNING}");
        return Ok(())
    }

    if cli.update {
        match http_fn::get_compatible_version(&server_url).await {
            Ok(required) => {
                if is_version_lower(CLIENT_VERSION, &required) {
                    println!("please update version (current: {CLIENT_VERSION}, required: {required})");
                } else {
                    println!("Client is up to date (version: {CLIENT_VERSION})");
                }
            }
            Err(e) => eprintln!("Failed to check version: {}", e),
        }
        return Ok(());
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
            http_fn::send_list(server_url).await;
        }
        Some(Commands::Add { name, token, model, brand }) => {
            match http_fn::add_agent_request(&name, &token, &model, &brand, server_url).await {
                Ok(response) => {
                    println!("{} (ID: {})", response.message, response.id);
                }
                Err(e) => {
                    eprintln!("{}: {}", MSG_FAIL_CREATE, e);
                }
            }
        }
        Some(Commands::Remove { id }) => {
            match http_fn::remove_agent_request(id, server_url).await {
                Ok(response) => {
                    println!("{}", response.message);
                }
                Err(e) => {
                    eprintln!("{}: {}", MSG_FAIL_REMOVE, e);
                }
            }
        }
        None => {}
    }
}
