use sqlx::SqlitePool;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

const PORT: u16 = 6411;

#[derive(Debug, Clone, sqlx::FromRow)]
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
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let pool = Arc::new(SqlitePool::connect("sqlite:agents.sqlite").await?);
    info!("Connected to database");

    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();
    let listener = TcpListener::bind(&addr).await?;

    info!("kserverd listening on {}", addr);

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("Accepted connection from {}", peer_addr);

        let pool_clone = Arc::clone(&pool);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, pool_clone).await {
                error!("Error handling connection from {}: {}", peer_addr, e);
            }
            info!("Connection closed from {}", peer_addr);
        });
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    pool: Arc<SqlitePool>,
) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    while let Some(line) = lines.next_line().await? {
        let command = line.trim();
        info!("Received command: {}", command);

        if command.is_empty() {
            continue;
        }

        if command == "quit" {
            writer.write_all(b"Goodbye.\n").await?;
            break;
        }

        let response = handle_command(command, &pool).await;
        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
        return Ok(());
    }

    Ok(())
}

async fn handle_command(cmd: &str, pool: &SqlitePool) -> String {
    match cmd {
        "ping" => "pong".to_string(),
        "list" => {
            let list_txt = "test";
            return format!("list of {list_txt}");
        }
        _ => {
            format!(
                "Unknown command: {}. Type 'help' for available commands.",
                cmd
            )
        }
    }
}

pub async fn list_agents(pool: &SqlitePool) -> Result<Vec<Agent>, sqlx::Error> {
    sqlx::query_as::<_, Agent>("SELECT id, name, token, model, created_at FROM agents")
        .fetch_all(pool)
        .await
}
