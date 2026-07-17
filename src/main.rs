use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod keeper;
mod client;
mod db;
mod message;

use keeper::FireKeeper;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let config_dir = dirs::config_dir().unwrap().join("ashenlink");
    std::fs::create_dir_all(&config_dir).unwrap();
    let db_path = config_dir.join("messages.db");
    let pool = db::init_db(db_path.to_str().unwrap()).await;
    
    let keeper = Arc::new(Mutex::new(FireKeeper::new(pool)));

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let fk = Arc::clone(&keeper);
        tokio::spawn(async move { client::handle_client(stream, fk).await });
    }
}
