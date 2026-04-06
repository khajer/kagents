use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};


#[derive(Parser)]
#[command(name = "kcli")]
#[command(author = "kagents")]
#[command(version)]
#[command(about = "KAgents CLI ", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
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
        #[arg(short, long, help = "Agent name")]
        name: String,
        #[arg(short, long, help = "Agent token")]
        token: String,
        #[arg(short, long, help = "Agent model")]
        model: String,
    },
    #[command(about = "remove agents")]
    Remove {
        #[arg(short, long, help = "Agent ID to remove")]
        id: i64,
    },
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
