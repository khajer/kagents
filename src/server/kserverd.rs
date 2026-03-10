use std::error::Error;
use std::net::SocketAddr;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

const PORT: u16 = 6411;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let addr: SocketAddr = ([0, 0, 0, 0], PORT).into();
    let listener = TcpListener::bind(&addr).await?;

    info!("kserverd listening on {}", addr);

    loop {
        let (stream, peer_addr) = listener.accept().await?;
        info!("Accepted connection from {}", peer_addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                error!("Error handling connection from {}: {}", peer_addr, e);
            }
            info!("Connection closed from {}", peer_addr);
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let (reader, mut writer) = stream.split();
    let mut lines = BufReader::new(reader).lines();

    writer
        .write_all(b"Welcome to kserverd. Enter commands (type 'quit' to disconnect).\n")
        .await?;

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

        let response = handle_command(command);
        writer.write_all(response.as_bytes()).await?;
        writer.write_all(b"\n").await?;
    }

    Ok(())
}

fn handle_command(cmd: &str) -> String {
    match cmd {
        "ping" => "pong".to_string(),
        "help" => "Available commands: ping, help, quit, echo <text>".to_string(),
        _ if cmd.starts_with("echo ") => {
            let text = &cmd[5..];
            text.to_string()
        }
        _ => {
            format!(
                "Unknown command: {}. Type 'help' for available commands.",
                cmd
            )
        }
    }
}
