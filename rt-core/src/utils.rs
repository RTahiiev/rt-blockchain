use crate::block::Block;
use rt_storage::get_chain;

fn create_first_block() -> Block {
    let data: String = "Hello, world!".to_owned();
    Block::new(data)
}

fn sync() {
    let chain = get_chain().unwrap();
    for block in chain {
        unimplemented!()
    }
}
