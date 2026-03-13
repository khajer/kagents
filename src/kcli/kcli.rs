use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "kcli")]
#[command(author = "kagents")]
#[command(version)]
#[command(about = "KAgents CLI - A task scheduler and runner", long_about = None)]
#[command(arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Start the scheduler daemon")]
    Start,

    #[command(about = "Run a specific task")]
    Run {
        #[arg(short, long, help = "Task name or ID to run")]
        task: String,
    },

    #[command(about = "List all scheduled tasks")]
    List {
        #[arg(short, long, help = "Show all tasks including completed")]
        all: bool,
    },

    #[command(about = "Show task or system status")]
    Status {
        #[arg(short, long, help = "Task name or ID")]
        task: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start) => {
            println!("Starting scheduler daemon...");
        }
        Some(Commands::Run { task }) => {
            println!("Running task: {}", task);
        }
        Some(Commands::List { all }) => {
            if all {
                println!("Listing all tasks (including completed)...");
            } else {
                println!("Listing pending tasks...");
            }
        }
        Some(Commands::Status { task }) => {
            if let Some(t) = task {
                println!("Status for task: {}", t);
            } else {
                println!("System status: OK");
            }
        }
        None => {
            println!("Use 'kcli --help' for usage information.");
        }
    }

    Ok(())
}
