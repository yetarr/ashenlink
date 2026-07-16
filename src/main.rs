use std::sync::Arc;

use tokio::net::TcpListener;
use tokio::sync::Mutex;

mod keeper;
mod client;

use keeper::FireKeeper;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let keeper = Arc::new(Mutex::new(FireKeeper::new()));

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let fk = Arc::clone(&keeper);
        tokio::spawn(async move { client::handle_client(stream, fk).await });
    }
}
