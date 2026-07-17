use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;
use anyhow::{Context, Result};

mod client;
mod db;
mod keeper;
mod message;

use keeper::FireKeeper;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let config_dir = dirs::config_dir().context("no config directory found")?.join("ashenlink");
    std::fs::create_dir_all(&config_dir)?;
    let db_path = config_dir.join("messages.db");
    let pool = db::init_db(db_path.to_str().context("db_path is not a valid unicode")?).await?;
    let keeper = Arc::new(Mutex::new(FireKeeper::new(pool)));

    loop {
        let (stream, _addr) = listener.accept().await?;
        let fk = Arc::clone(&keeper);
        tokio::spawn(async move {
            if let Err(e) = client::handle_client(stream, fk).await {
                eprintln!("client error: {:?}", e);
            }
        });
    }
}
