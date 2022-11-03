use std::io::Write;

use chrono::Utc;
use crypto_hash::{Algorithm, Hasher};

/// Main structure.
/// Describe block.
#[derive(Debug)]
struct Block {
    index: usize,
    timestamp: i64,
    previous_hash: Vec<u8>,
    current_hash: Vec<u8>,
    data: String,
}

impl Block {
    pub fn new(data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        // TODO: Read more about hash...
        let mut hasher = Hasher::new(Algorithm::SHA256);
        hasher.write_all(&timestamp.to_ne_bytes());
        let current_hash = hasher.finish();

        unimplemented!()
    }
}
