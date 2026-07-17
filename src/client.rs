use std::sync::Arc;

use anyhow::Result;
use futures_util::StreamExt;
use futures_util::stream::SplitStream;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{
    WebSocketStream, accept_async,
    tungstenite::{self, Utf8Bytes},
};

use crate::{keeper::FireKeeper, message::ShrineMessage};

pub struct ShrineClient {
    pub id: usize,
    pub name: String,
    pub stream: SplitStream<WebSocketStream<TcpStream>>,
}

impl ShrineClient {
    pub fn new(id: usize, stream: SplitStream<WebSocketStream<TcpStream>>) -> Self {
        ShrineClient {
            id,
            stream,
            name: String::new(),
        }
    }

    pub fn has_name(&self) -> bool {
        !self.name.is_empty()
    }

    pub fn name(&self) -> String {
        if self.has_name() {
            self.name.clone()
        } else {
            format!("{}{}", "Anonymous", self.id)
        }
    }
}

enum Signal {
    Message(Utf8Bytes),
    Stop,
}

pub async fn handle_client(stream: TcpStream, keeper: Arc<Mutex<FireKeeper>>) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    let mut client = keeper.lock().await.recognize(ws_stream).await;
    loop {
        let response = read_msg(&mut client.stream).await;
        match response {
            Some(Signal::Message(msg)) => {
                let msg = ShrineMessage::new(&msg, &client)?;
                println!("{}", &msg);
                keeper.lock().await.broadcast(&msg).await?;
            }
            Some(Signal::Stop) => {
                keeper.lock().await.forget(&client).await;
                break;
            }
            _ => {}
        }
    }

    Ok(())
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
