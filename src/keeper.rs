use std::collections::HashMap;

use tokio::net::tcp::OwnedWriteHalf;
use tokio::io::AsyncWriteExt;

pub struct FireKeeper {
    streams: HashMap<usize, OwnedWriteHalf>,
    next_id: usize,
}

impl FireKeeper {
    pub fn new() -> FireKeeper {
        FireKeeper {
            streams: HashMap::new(),
            next_id: 1,
        }
    }

    pub async fn recognize(&mut self, stream: OwnedWriteHalf) -> usize {
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
            stream.write_all(msg.as_bytes()).await.unwrap();
        }

        println!("Keeper: {}", content);
    }

    pub async fn broadcast(&mut self, content: &str, from: usize) {
        for kp in &mut self.streams {
            if from == *kp.0 {
                continue;
            }

            let msg = format!("{}: {}\n", from, content.trim_end());
            kp.1.write_all(msg.as_bytes()).await.unwrap();
        }
    }
}