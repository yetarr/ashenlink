use anyhow::Result;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn init_db(path: &str) -> Result<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite://{}?mode=rwc", path))
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_name TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn save_message(pool: &SqlitePool, sender_name: &str, content: &str) -> Result<()> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

    sqlx::query("INSERT INTO messages (sender_name, content, timestamp) VALUES (?, ?, ?)")
        .bind(sender_name)
        .bind(content)
        .bind(timestamp)
        .execute(pool)
        .await?;

    Ok(())
}
