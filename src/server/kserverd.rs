use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::error::Error;
use tokio::fs;
use tracing::{error, info};

const PORT: u16 = 6411;
const AGENTS_FOLDER: &str = "workspace";



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

#[derive(Deserialize)]
struct CreateAgent {
    name: String,
    token: String,
    model: String,
}

#[derive(Serialize)]
struct CreateAgentResponse {
    id: i64,
    message: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct RemoveAgent {
    id: i64,
}

#[derive(Serialize)]
struct RemoveAgentResponse {
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let pool = SqlitePool::connect("sqlite:agents.sqlite").await?;
    info!("Connected to database");

    create_table_if_not_exists(&pool).await?;
    info!("Database table initialized");

    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/list", get(list_handler))
        .route("/add", post(add_agent_handler))
        .route("/remove", delete(remove_agent_handler))
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

async fn list_handler(State(pool): State<SqlitePool>) -> Result<Json<ListResponse>, (StatusCode, Json<ErrorResponse>)> {
    match list_agents(&pool).await {
        Ok(agents) => Ok(Json(ListResponse { agents })),
        Err(e) => {
            error!("Failed to list agents: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to list agents: {}", e),
                }),
            ))
        }
    }
}

async fn add_agent_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<CreateAgent>,
) -> Result<Json<CreateAgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    let created_at = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO agents (name, token, model, created_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&payload.name)
    .bind(&payload.token)
    .bind(&payload.model)
    .bind(&created_at)
    .execute(&pool)
    .await;

    match result {
        Ok(query_result) => {
            // Create folder with agent's name under agents directory
            let folder_path = format!("./{}/{}", AGENTS_FOLDER, payload.name);
            if let Err(e) = fs::create_dir_all(&folder_path).await {
                error!("Failed to create folder for agent {}: {}", payload.name, e);
            } else {
                info!("Created folder for agent: {}", folder_path);
            }

            Ok(Json(CreateAgentResponse {
                id: query_result.last_insert_rowid(),
                message: "Agent created successfully".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to create agent: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to create agent: {}", e),
                }),
            ))
        }
    }
}

async fn remove_agent_handler(
    State(pool): State<SqlitePool>,
    Json(payload): Json<RemoveAgent>,
) -> Result<Json<RemoveAgentResponse>, (StatusCode, Json<ErrorResponse>)> {
    // First, get the agent's name before deleting
    let agent_result = sqlx::query_as::<_, Agent>("SELECT id, name, token, model, created_at FROM agents WHERE id = ?")
        .bind(payload.id)
        .fetch_optional(&pool)
        .await;

    match agent_result {
        Ok(Some(agent)) => {
            // Delete the folder
            let folder_path = format!("./{}/{}", AGENTS_FOLDER, agent.name);
            if let Err(e) = fs::remove_dir_all(&folder_path).await {
                error!("Failed to remove folder for agent {}: {}", agent.name, e);
                // Continue with database deletion even if folder removal fails
            } else {
                info!("Removed folder for agent: {}", folder_path);
            }

            // Delete from database
            let delete_result = sqlx::query("DELETE FROM agents WHERE id = ?")
                .bind(payload.id)
                .execute(&pool)
                .await;

            match delete_result {
                Ok(_) => {
                    Ok(Json(RemoveAgentResponse {
                        message: format!("Agent {} removed successfully", payload.id),
                    }))
                }
                Err(e) => {
                    error!("Failed to remove agent from database: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: format!("Failed to remove agent: {}", e),
                        }),
                    ))
                }
            }
        }
        Ok(None) => {
            Err((
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: format!("Agent {} not found", payload.id),
                }),
            ))
        }
        Err(e) => {
            error!("Failed to query agent: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("Failed to query agent: {}", e),
                }),
            ))
        }
    }
}


pub async fn list_agents(pool: &SqlitePool) -> Result<Vec<Agent>, sqlx::Error> {
    sqlx::query_as::<_, Agent>("SELECT id, name, token, model, created_at FROM agents")
        .fetch_all(pool)
        .await
}

async fn create_table_if_not_exists(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS agents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            token TEXT NOT NULL,
            model TEXT NOT NULL,
            created_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}
