use sqlx::SqlitePool;
use std::error::Error;

pub async fn create_table_if_not_exists(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
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
