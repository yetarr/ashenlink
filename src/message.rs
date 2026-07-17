use std::{fmt::Display, time::{SystemTime, UNIX_EPOCH},};

use anyhow::Result;
use tokio_tungstenite::tungstenite::Utf8Bytes;

use crate::client::ShrineClient;

pub struct ShrineMessage {
    pub content: String,
    pub sender_id: usize,
    pub sender_name: String,
    pub timestamp: i64,
}

impl ShrineMessage {
    pub fn new(content: &str, sender: &ShrineClient) -> Result<Self> {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;

        Ok(ShrineMessage {
            content: content.trim_end().to_string(),
            sender_id: sender.id,
            sender_name: sender.name().trim().to_string(),
            timestamp,
        })
    }
}

impl Display for ShrineMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.sender_name, self.content)
    }
}

impl Into<Utf8Bytes> for &ShrineMessage {
    fn into(self) -> Utf8Bytes {
        self.to_string().into()
    }
}
