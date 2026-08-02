use chrono::Utc;
use sqlx::SqlitePool;
use std::error::Error;

use crate::handler::{Agent, CreateAgent};

const SQL_CREATE_PROMPT_TABLE: &str = "CREATE TABLE IF NOT EXISTS prompts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    prompt TEXT NOT NULL,
    agent_id INTEGER,
    user_prompt BOOLEAN NOT NULL,
    created_at TEXT NOT NULL
)";
const SQL_INSERT_PROMPT: &str = "INSERT INTO prompts (prompt, agent_id, user_prompt, created_at) VALUES (?, ?, ?, ?)";

const SQL_SELECT_AGENT_ALL: &str = "SELECT id, name, token, model, brand, status, created_at FROM agents";
const SQL_SELECT_AGENT_BY_ID: &str = "SELECT id, name, token, model, brand, status, created_at FROM agents WHERE id = ?";
const SQL_DELETE_AGENT_BY_ID: &str = "DELETE FROM agents WHERE id = ?";
const SQL_INSERT_AGENT: &str = "INSERT INTO agents (name, token, model, brand, status, created_at) VALUES (?, ?, ?, ?, ?, ?)";
const SQL_CREATE_AGENT_TABLE: &str = "CREATE TABLE IF NOT EXISTS agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    token TEXT NOT NULL,
    model TEXT NOT NULL,
    brand TEXT NOT NULL,
    status TEXT NOT NULL,
    created_at TEXT NOT NULL
)";

pub async fn create_table_if_not_exists(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    sqlx::query(SQL_CREATE_AGENT_TABLE)
        .execute(pool)
        .await?;

    sqlx::query(SQL_CREATE_PROMPT_TABLE)
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
        .bind(&payload.brand)
        .bind(&payload.status)
        .bind(&created_at)
        .execute(pool)
        .await
}

pub async fn insert_prompt(pool: &SqlitePool, agent_id: Option<i64>, prompt: &str, user_prompt: bool) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let created_at = Utc::now().to_rfc3339();
    sqlx::query(SQL_INSERT_PROMPT)
        .bind(prompt)
        .bind(agent_id)
        .bind(user_prompt)
        .bind(&created_at)
        .execute(pool)
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn memory_pool() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        create_table_if_not_exists(&pool).await.unwrap();
        pool
    }

    fn sample_agent() -> CreateAgent {
        CreateAgent {
            name: "bot1".to_string(),
            token: "tok".to_string(),
            model: "gpt-4".to_string(),
            brand: "openai".to_string(),
            status: "active".to_string(),
        }
    }

    #[tokio::test]
    async fn insert_and_list_agent() {
        let pool = memory_pool().await;
        insert_agent(&pool, &sample_agent()).await.unwrap();
        let agents = list_agents(&pool).await.unwrap();
        assert_eq!(agents.len(), 1);
        assert_eq!(agents[0].name, "bot1");
        assert_eq!(agents[0].brand, "openai");
    }

    #[tokio::test]
    async fn get_agent_by_id_found_and_missing() {
        let pool = memory_pool().await;
        let result = insert_agent(&pool, &sample_agent()).await.unwrap();
        let id = result.last_insert_rowid();

        let found = get_agent_by_id(&pool, id).await.unwrap();
        assert_eq!(found.unwrap().name, "bot1");

        let missing = get_agent_by_id(&pool, id + 999).await.unwrap();
        assert!(missing.is_none());
    }

    #[tokio::test]
    async fn delete_agent_removes_row() {
        let pool = memory_pool().await;
        let result = insert_agent(&pool, &sample_agent()).await.unwrap();
        let id = result.last_insert_rowid();

        delete_agent_by_id(&pool, id).await.unwrap();
        assert!(get_agent_by_id(&pool, id).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn insert_prompt_succeeds() {
        let pool = memory_pool().await;
        let result = insert_prompt(&pool, None, "hello", true).await.unwrap();
        assert_eq!(result.rows_affected(), 1);
    }
}
