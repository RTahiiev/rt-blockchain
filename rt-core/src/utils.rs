use crate::block::Block;
use chrono::Utc;

fn create_first_block() -> Block {
    let data: String = "Hello, world!".to_owned();
    Block::new(data)
}

