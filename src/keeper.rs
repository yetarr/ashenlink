use std::collections::HashMap;

use futures_util::SinkExt;
use tokio::net::TcpStream;
use futures_util::stream::SplitSink;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub struct FireKeeper {
    streams: HashMap<usize, SplitSink<WebSocketStream<TcpStream>, Message>>,
    next_id: usize,
}

impl FireKeeper {
    pub fn new() -> FireKeeper {
        FireKeeper {
            streams: HashMap::new(),
            next_id: 1,
        }
    }

    pub async fn recognize(&mut self, stream: SplitSink<WebSocketStream<TcpStream>, Message>) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.streams.insert(id, stream);
        self.warn(&format!("{} entered the shrine!", id)).await;

        id
    }

    pub async fn forget(&mut self, id: usize) {
        self.streams.remove(&id);
        self.warn(&format!("{} left the shrine!", id)).await;
    }

    async fn warn(&mut self, content: &str) {
        for (_, stream) in &mut self.streams {
            let msg = format!("Keeper: {}\n", content);
            stream.send(Message::Text(msg.into())).await.unwrap();
        }

        println!("Keeper: {}", content);
    }

    pub async fn broadcast(&mut self, content: &str, from: usize) {
        for kp in &mut self.streams {
            if from == *kp.0 {
                continue;
            }

            let msg = format!("{}: {}", from, content);
            kp.1.send(Message::Text(msg.into())).await.unwrap();
        }
    }
}