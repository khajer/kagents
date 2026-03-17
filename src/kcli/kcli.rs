use clap::{Parser, Subcommand};

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

const SERVER_URL: &str = "127.0.0.1:6411";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {

        Some(Commands::Status { task }) => {

            if check_server_open(SERVER_URL).await != true {
                println!("the server doesn't run.");

                return Ok(())
            }
            if let Some(t) = task {
                println!("Status for task: {}", t);
            } else {
                println!("System status: OK");
            }
        }
        Some(Commands::List { task: _ }) => {
            println!("list agent works ");
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

async fn check_server_open(server_url: &str) -> bool {
    tokio::net::TcpStream::connect(server_url).await.is_ok()
}
