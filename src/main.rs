use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

struct FireKeeper {
    streams: HashMap<usize, TcpStream>,
    next_id: usize,
}

impl FireKeeper {
    fn new() -> FireKeeper {
        FireKeeper {
            streams: HashMap::new(),
            next_id: 1,
        }
    }

    fn recognize(&mut self, stream: TcpStream) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        self.streams.insert(id, stream);
        self.warn(&format!("{} entered the shrine!", id));

        id
    }

    fn forget(&mut self, id: usize) {
        self.streams.remove(&id);
        self.warn(&format!("{} left the shrine!", id));
    }

    fn warn(&mut self, content: &str) {
        for (_, stream) in &mut self.streams {
            let msg = format!("Keeper: {}\n", content);
            stream.write_all(msg.as_bytes()).unwrap();
        }

        println!("Keeper: {}", content);
    }

    fn broadcast(&mut self, content: &str, from: usize) {
        for kp in &mut self.streams {
            if from == *kp.0 {
                continue;
            }
            
            let msg = format!("{}: {}\n", from, content.trim_end());
            kp.1.write_all(msg.as_bytes()).unwrap();
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let keeper = Arc::new(Mutex::new(FireKeeper::new()));

    loop {
        let (stream, _addr) = listener.accept().unwrap();
        let fk = Arc::clone(&keeper);
        thread::spawn(move || {
            let reader_stream = stream.try_clone().unwrap();
            let id = fk.lock().unwrap().recognize(stream);
            let mut reader = BufReader::new(reader_stream);
            loop {
                let mut ln = String::new();
                let len = reader.read_line(&mut ln).unwrap();
                if len == 0 {
                    fk.lock().unwrap().forget(id);
                    break;
                }
                println!("{}: {}", id, ln.trim_end());
                fk.lock().unwrap().broadcast(&ln, id);
            }
        });
    }
}
