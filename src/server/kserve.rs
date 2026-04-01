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
use handler::list_handler;
use handler::process_handler;
use handler::add_agent_handler;
use handler::remove_agent_handler;

const PORT: u16 = 6411;
const SQLITE_FILE: &str = "sqlite:agents.sqlite";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let pool = SqlitePool::connect(SQLITE_FILE).await?;
    info!("Connected to database");

    create_table_if_not_exists(&pool).await?;
    info!("Database table initialized");

    let app = Router::new()
        .route("/list", get(list_handler))
        .route("/process", get(process_handler))
        .route("/add", post(add_agent_handler))
        .route("/remove", delete(remove_agent_handler))
        .with_state(pool);

    let addr: std::net::SocketAddr = ([0, 0, 0, 0], PORT).into();

    info!("kserve HTTP server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
