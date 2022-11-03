use std::fs;
use std::env;
use std::path::Path;

const BASIC_STORAGE_DIR_NAME: &str = "/chaindata";

pub fn init_storage() -> std::io::Result<()> { 
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let chain_data = env::join_paths([current_dir, base_dir].iter()).unwrap();
    fs::create_dir_all(chain_data)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
