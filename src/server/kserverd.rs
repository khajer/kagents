use axum::{
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::error::Error;

use tracing::{info};

const PORT: u16 = 6411;

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
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

#[derive(Serialize)]
struct PingResponse {
    message: String,
}

#[derive(Serialize)]
struct ListResponse {
    agents: Vec<Agent>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:agents.sqlite").await?;
    info!("Connected to database");

    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/list", get(list_handler))
        .with_state(pool);

    let addr: std::net::SocketAddr = ([0, 0, 0, 0], PORT).into();

    info!("kserverd HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ping_handler() -> impl IntoResponse {
    Json(PingResponse {
        message: "pong".to_string(),
    })
}

async fn list_handler() -> impl IntoResponse {
    Json(ListResponse {
        agents: vec![],
    })
}


pub async fn list_agents(pool: &SqlitePool) -> Result<Vec<Agent>, sqlx::Error> {
    sqlx::query_as::<_, Agent>("SELECT id, name, token, model, created_at FROM agents")
        .fetch_all(pool)
        .await
}
