use std::fmt::Debug;
use std::fs;
use std::env;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error};

const BASIC_STORAGE_DIR_NAME: &str = "/chaindata";

pub fn init_storage() -> std::io::Result<()> { 
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let chain_data = env::join_paths([current_dir, base_dir].iter()).unwrap();
    fs::create_dir_all(chain_data)?;
    Ok(())
}

pub fn write_data<T: Debug>(name: String, data: T) -> Result<(), Error> {
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let name = Path::new(&name);
    let chain_data = env::join_paths([current_dir, base_dir, name].iter()).unwrap();
    let mut file = fs::File::create(chain_data)?;
    write!(file, "{:?}", data)?;

    Ok(())
}

pub fn read_data(name: String) -> Result<Vec<String>, Error> {
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let name = Path::new(&name);
    let chain_data = env::join_paths([current_dir, base_dir, name].iter()).unwrap();
    let file = fs::File::open(chain_data)?;
    let buffered = BufReader::new(file);
    let mut data: Vec<String> = Vec::new();
    for line in buffered.lines() {
        data.push(line?);
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!()
    }
}
