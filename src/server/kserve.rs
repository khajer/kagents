use axum::{
    routing::{delete, get, post},
    Router,
};
use sqlx::SqlitePool;
use std::error::Error;
use tracing::info;

mod db_func;
use db_func::create_table_if_not_exists;

mod handler;
mod call_agent;
use handler::list_handler;
use handler::process_handler;
use handler::add_agent_handler;
use handler::remove_agent_handler;
use handler::prompt_handler;
use handler::compatible_client_version_handler;
use handler::ping_handler;

const PORT: u16 = 6411;

const SQLITE_FILENAME: &str = "agents.sqlite";
const SQLITE_CONN: &str = "sqlite:agents.sqlite";
const MSG_SERVER_STARTED: &str = "kserve HTTP server listening on ";
const MSG_CREATED_DB: &str = "created file database";
const MSG_DB_INITIALIZED: &str = "Database table initialized";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    if !std::path::Path::new(SQLITE_FILENAME).exists() {
        println!("{MSG_CREATED_DB}");
        std::fs::File::create(SQLITE_FILENAME)?;
    }
    let pool = SqlitePool::connect(SQLITE_CONN).await?;
    create_table_if_not_exists(&pool).await?;
    info!("{MSG_DB_INITIALIZED}");

    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/list", get(list_handler))
        .route("/process", get(process_handler))
        .route("/prompt", get(prompt_handler))
        .route("/add", post(add_agent_handler))
        .route("/remove", delete(remove_agent_handler))
        .route("/compatible_client_version", get(compatible_client_version_handler))
        .with_state(pool);

    let addr: std::net::SocketAddr = ([0, 0, 0, 0], PORT).into();

    info!("{MSG_SERVER_STARTED} {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
