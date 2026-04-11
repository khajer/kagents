use chrono::Utc;
use sqlx::SqlitePool;
use std::error::Error;

use crate::handler::{Agent, CreateAgent};

const SQL_SELECT_AGENT_ALL: &str = "SELECT id, name, token, model, status, created_at FROM agents";
const SQL_SELECT_AGENT_BY_ID: &str = "SELECT id, name, token, model, status, created_at FROM agents WHERE id = ?";
const SQL_DELETE_AGENT_BY_ID: &str = "DELETE FROM agents WHERE id = ?";
const SQL_INSERT_AGENT: &str = "INSERT INTO agents (name, token, model, status, created_at) VALUES (?, ?, ?, ?, ?)";

pub async fn create_table_if_not_exists(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS agents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            token TEXT NOT NULL,
            model TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TEXT NOT NULL
        )"
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn list_agents(pool: &SqlitePool) -> Result<Vec<Agent>, sqlx::Error> {
    sqlx::query_as::<_, Agent>(SQL_SELECT_AGENT_ALL)
        .fetch_all(pool)
        .await
}

pub async fn get_agent_by_id(pool: &SqlitePool, id: i64) -> Result<Option<Agent>, sqlx::Error> {
    sqlx::query_as::<_, Agent>(SQL_SELECT_AGENT_BY_ID)
        .bind(id)
        .fetch_optional(pool)
        .await
}

pub async fn delete_agent_by_id(pool: &SqlitePool, id: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    sqlx::query(SQL_DELETE_AGENT_BY_ID)
        .bind(id)
        .execute(pool)
        .await
}

pub async fn insert_agent(pool: &SqlitePool, payload: &CreateAgent) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let created_at = Utc::now().to_rfc3339();
    sqlx::query(SQL_INSERT_AGENT)
        .bind(&payload.name)
        .bind(&payload.token)
        .bind(&payload.model)
        .bind(&payload.status)
        .bind(&created_at)
        .execute(pool)
        .await
}
