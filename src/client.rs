use std::sync::Arc;

use tokio::{net::TcpStream, sync::Mutex};
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::keeper::FireKeeper;

pub async fn handle_client(stream: TcpStream, keeper: Arc<Mutex<FireKeeper>>) {
    let (reader_stream, write_stream) = stream.into_split();
    let id = keeper.lock().await.recognize(write_stream).await;
    let mut reader = BufReader::new(reader_stream);
    loop {
        let mut ln = String::new();
        let len = reader.read_line(&mut ln).await.unwrap();
        if len == 0 {
            keeper.lock().await.forget(id).await;
            break;
        }
        println!("{}: {}", id, ln.trim_end());
        keeper.lock().await.broadcast(&ln, id).await;
    }
}