use std::collections::HashMap;

use anyhow::Result;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};
use sqlx::SqlitePool;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::tungstenite::Message;

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

    pub async fn recognize(&mut self, stream: WebSocketStream<TcpStream>) -> Result<ShrineClient> {
        let (mut write, read) = stream.split();

        let id = self.next_id;
        self.next_id += 1;

        self.inform_context(&mut write).await?;
        self.streams.insert(id, write);

        let client = ShrineClient::new(id, read);
        self.warn(&format!("{} entered the shrine!", client.name())).await;
        Ok(client)
    }

    pub async fn inform_context(&self, write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,) -> Result<()> {
        let mut history = db::recent_messages(&self.pool, 20).await?;
        history.reverse();
        for (sender, content) in history {
            write.send(Message::Text(format!("[context] {}: {}", sender, content).into())).await?;
        }
        Ok(())
    }

    pub async fn forget(&mut self, client: &ShrineClient) -> Result<()> {
        self.streams.remove(&client.id);
        self.warn(&format!("{} left the shrine!", client.name())).await;
        Ok(())
    }

    async fn warn(&mut self, content: &str) {
        for (_, stream) in &mut self.streams {
            let msg = format!("Keeper: {}\n", content);
            if let Err(e) = stream.send(Message::Text(msg.into())).await {
                eprintln!("warn error: {:?}", e);
            }
        }
        println!("Keeper: {}", content);
    }

    pub async fn broadcast(&mut self, msg: &ShrineMessage) -> Result<()> {
        for kp in &mut self.streams {
            if msg.sender_id == *kp.0 {
                continue;
            }
            kp.1.send(Message::Text(msg.into())).await?;
        }
        db::save_message(&self.pool, &msg.sender_name, &msg.content).await?;
        Ok(())
    }
}
