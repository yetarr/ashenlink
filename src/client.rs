use std::sync::Arc;

use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{WebSocketStream, accept_async, tungstenite::{self, Utf8Bytes}};

use crate::keeper::FireKeeper;

enum Signal {
    Message(Utf8Bytes),
    Stop
}

pub async fn handle_client(stream: TcpStream, keeper: Arc<Mutex<FireKeeper>>) {
    let ws_stream = accept_async(stream).await.unwrap();
    let (write, mut read) = ws_stream.split();
    let id = keeper.lock().await.recognize(write).await;
    loop {
        let response = read_msg(&mut read).await;
        match response {
            Some(Signal::Message(msg)) => { 
                println!("{}: {}", id, msg.trim_end());
                keeper.lock().await.broadcast(&msg, id).await;
            },
            Some(Signal::Stop) => {
                keeper.lock().await.forget(id).await;
                break;
            },
            _ => {},
        }

    }
}

async fn read_msg(reader: &mut SplitStream<WebSocketStream<TcpStream>>) -> Option<Signal> {
    if let Some(msg) = reader.next().await {
        let msg = match msg {
            Ok(m) => m,
            Err(_) => return Some(Signal::Stop),
        };
        
        match msg {
            tungstenite::Message::Text(text) => Some(Signal::Message(text)),
            tungstenite::Message::Close(_) => Some(Signal::Stop),
            _ => None,
        }
    } else {
        None
    }
}