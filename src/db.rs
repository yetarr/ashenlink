use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::time::{SystemTime, UNIX_EPOCH};

pub async fn init_db(path: &str) -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .connect(&format!("sqlite://{}?mode=rwc", path))
        .await
        .unwrap();

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender_name TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )"
    )
    .execute(&pool)
    .await
    .unwrap();

    pool
}

pub async fn save_message(pool: &SqlitePool, sender_name: &str, content: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    sqlx::query("INSERT INTO messages (sender_name, content, timestamp) VALUES (?, ?, ?)")
        .bind(sender_name)
        .bind(content)
        .bind(timestamp)
        .execute(pool)
        .await
        .unwrap();
}