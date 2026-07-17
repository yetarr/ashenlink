use std::collections::HashMap;

use futures_util::{SinkExt, StreamExt};
use sqlx::SqlitePool;
use tokio::net::TcpStream;
use futures_util::stream::SplitSink;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::client::ShrineClient;
use crate::db;
use crate::message::ShrineMessage;

pub struct FireKeeper {
    streams: HashMap<usize, SplitSink<WebSocketStream<TcpStream>, Message>>,
    next_id: usize,
    pool: SqlitePool,
}

impl FireKeeper {
    pub fn new(pool: SqlitePool) -> FireKeeper {
        FireKeeper {
            streams: HashMap::new(),
            next_id: 1,
            pool,
        }
    }

    pub async fn recognize(&mut self, stream: WebSocketStream<TcpStream>) -> ShrineClient {
        let (write, read) = stream.split();
    
        let id = self.next_id;
        self.next_id += 1;
        self.streams.insert(id, write);
        
        let client = ShrineClient::new(id, read);
        self.warn(&format!("{} entered the shrine!", client.name())).await;
        client
    }

    pub async fn forget(&mut self, client: &ShrineClient) {
        self.streams.remove(&client.id);
        self.warn(&format!("{} left the shrine!", client.name())).await;
    }

    async fn warn(&mut self, content: &str) {
        for (_, stream) in &mut self.streams {
            let msg = format!("Keeper: {}\n", content);
            stream.send(Message::Text(msg.into())).await.unwrap();
        }

        println!("Keeper: {}", content);
    }

    pub async fn broadcast(&mut self, msg: &ShrineMessage) {
        for kp in &mut self.streams {
            if msg.sender_id == *kp.0 {
                continue;
            }

            kp.1.send(Message::Text(msg.into())).await.unwrap();
        }

        db::save_message(&self.pool, &msg.sender_name, &msg.content).await;
    }
}