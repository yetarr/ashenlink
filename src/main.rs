use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;

struct FireKeeper {
    streams: HashMap<usize, OwnedWriteHalf>,
    next_id: usize,
}

impl FireKeeper {
    fn new() -> FireKeeper {
        FireKeeper {
            streams: HashMap::new(),
            next_id: 1,
        }
    }

    async fn recognize(&mut self, stream: OwnedWriteHalf) -> usize {
        let id = self.next_id;
        self.next_id += 1;

        self.streams.insert(id, stream);
        self.warn(&format!("{} entered the shrine!", id)).await;

        id
    }

    async fn forget(&mut self, id: usize) {
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

    async fn broadcast(&mut self, content: &str, from: usize) {
        for kp in &mut self.streams {
            if from == *kp.0 {
                continue;
            }

            let msg = format!("{}: {}\n", from, content.trim_end());
            kp.1.write_all(msg.as_bytes()).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let keeper = Arc::new(Mutex::new(FireKeeper::new()));

    loop {
        let (stream, _addr) = listener.accept().await.unwrap();
        let fk = Arc::clone(&keeper);
        tokio::spawn(async move {
            let (reader_stream, write_stream) = stream.into_split();
            let id = fk.lock().await.recognize(write_stream).await;
            let mut reader = BufReader::new(reader_stream);
            loop {
                let mut ln = String::new();
                let len = reader.read_line(&mut ln).await.unwrap();
                if len == 0 {
                    fk.lock().await.forget(id).await;
                    break;
                }
                println!("{}: {}", id, ln.trim_end());
                fk.lock().await.broadcast(&ln, id).await;
            }
        });
    }
}
