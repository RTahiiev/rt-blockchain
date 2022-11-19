use std::io::Write;

use chrono::Utc;
use crypto_hash::{Algorithm, Hasher};
use rt_storage::write_data;

/// Main structure.
/// Describe block.
#[derive(Debug)]
pub struct Block {
    index: usize,
    timestamp: i64,
    previous_hash: Option<Vec<u8>>,
    current_hash: Vec<u8>,
    data: String,
}

impl Block {
    pub fn new(data: String) -> Self {
        let timestamp = Utc::now().timestamp();
        // TODO: Read more about hash...
        let mut hasher = Hasher::new(Algorithm::SHA256);
        hasher.write_all(&timestamp.to_ne_bytes()).map_err(
            |err| {
                log::error!("{:?}", err);
            }
        );
        let current_hash = hasher.finish();
        let index: usize = 0;
        log::info!("Block #{} was created.", index);
        Self {
            index, // TODO: auto increment
            timestamp,
            previous_hash: None,
            current_hash,
            data
        }
    }

    pub fn save(self) {
        let name: String = self.index.to_string();
        write_data(name, self).unwrap();
    }
}
