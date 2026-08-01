use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "kcli")]
#[command(author = "kagents")]
#[command(version)]
#[command(about = "KAgents CLI ", long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    #[arg(short = 'U', long, help = "Check if client version is up to date")]
    pub update: bool,
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
        #[arg(short, long, help = "Agent brand (e.g. openai, anthropic)")]
        brand: String,
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
    #[allow(dead_code)]
    pub token: String,
    pub model: String,
    pub brand: String,
    pub created_at: String,
}

impl std::fmt::Display for Agent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {} | Name: {} | Model: {} | Brand: {} | Created: {}",
            self.id, self.name, self.model, self.brand, self.created_at
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_definition_is_valid() {
        // catches clap misconfiguration (dupe flags, bad defaults, etc.)
        <Cli as clap::CommandFactory>::command().debug_assert();
    }

    #[test]
    fn parses_add_command() {
        let cli = Cli::try_parse_from([
            "kcli", "add", "-n", "bot1", "-t", "tok", "-m", "gpt-4", "-b", "openai",
        ])
        .unwrap();
        match cli.command {
            Some(Commands::Add { name, token, model, brand }) => {
                assert_eq!(name, "bot1");
                assert_eq!(token, "tok");
                assert_eq!(model, "gpt-4");
                assert_eq!(brand, "openai");
            }
            _ => panic!("expected Add command"),
        }
    }

    #[test]
    fn parses_remove_command() {
        let cli = Cli::try_parse_from(["kcli", "remove", "-i", "7"]).unwrap();
        match cli.command {
            Some(Commands::Remove { id }) => assert_eq!(id, 7),
            _ => panic!("expected Remove command"),
        }
    }

    #[test]
    fn parses_update_flag() {
        let cli = Cli::try_parse_from(["kcli", "-U"]).unwrap();
        assert!(cli.update);
        assert!(cli.command.is_none());
    }

    #[test]
    fn agent_display_format() {
        let agent = Agent {
            id: 1,
            name: "bot1".to_string(),
            token: "secret".to_string(),
            model: "gpt-4".to_string(),
            brand: "openai".to_string(),
            created_at: "2026-08-06T00:00:00Z".to_string(),
        };
        assert_eq!(
            agent.to_string(),
            "ID: 1 | Name: bot1 | Model: gpt-4 | Brand: openai | Created: 2026-08-06T00:00:00Z"
        );
    }
}
