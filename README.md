# Ashenlink

A TCP chat server written in Rust, built as a **learning project**, not a production tool.

This isn't meant to be a polished, ready-to-use chat app. It's a way to actually learn networking and concurrency in Rust by building something real, one phase at a time, instead of reading through it linearly. Each phase adds one new concept on top of the last.

## Learning Phases

- **Phase 1 - Sockets basics**: `TcpListener`, `TcpStream`, accepting a single connection, reading one line with `BufReader`.
- **Phase 2 - Handling multiple clients (sequentially)**: looping `accept()` to handle clients one after another, understanding the OS-level backlog queue and why a single blocking loop can't serve two clients at once.
- **Phase 3 - Concurrency with threads**: `std::thread::spawn` + `move` closures, giving each client its own thread so one client can't block another.
- **Phase 4 - Shared state across threads**: `Arc<Mutex<T>>` to safely share a registry of connected clients across threads, enabling actual message broadcasting between clients.
- **Phase 5+ (planned)**: revisiting this same problem with `async`/`tokio` instead of OS threads, to compare the two concurrency models directly.

## Usage

```bash
cargo run
```

Connect with `nc` (or any raw TCP client):

```bash
nc 127.0.0.1 8080
```

Messages sent by one connected client are broadcast to all other connected clients. Join/leave events are announced to everyone.

## Why this exists

This project is intentionally rough around the edges in places, it's a running record of learning networking and concurrency in Rust hands-on, not a finished product. Expect the code and this README to evolve as new phases are added.
