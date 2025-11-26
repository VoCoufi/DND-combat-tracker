use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub round: u32,
    pub message: String,
    pub timestamp: u64,
}

impl LogEntry {
    pub fn new(round: u32, message: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        Self {
            round,
            message,
            timestamp,
        }
    }
}
