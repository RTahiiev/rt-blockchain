use std::fmt::{Debug, Display};
use std::fs;
use std::env;
use std::path::Path;
use std::io::{Write, BufReader, BufRead, Error};

const BASIC_STORAGE_DIR_NAME: &str = "/chaindata";

pub fn init_storage() -> std::io::Result<()> {
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let chain_data = env::join_paths([current_dir, base_dir].iter()).unwrap();
    log::info!("Init storage: {:?}", chain_data);
    fs::create_dir_all(chain_data)?;
    Ok(())
}

pub fn write_data<T: Display>(name: String, data: T) -> Result<(), Error> {
    init_storage(); // TODO: error handle
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let name = Path::new(&name);
    let chain_data = env::join_paths([current_dir, base_dir, name].iter()).unwrap();
    log::info!("Write in: {:?}", chain_data);
    let mut file = fs::File::create(chain_data)?;
    write!(file, "{}", data)?;

    Ok(())
}

pub fn get_chain() -> Result<Vec<Vec<String>>, Error> {
    let mut chain: Vec<Vec<String>> = Vec::new();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let paths = fs::read_dir(base_dir).unwrap();
    for path in paths {
        let file = fs::File::open(path.unwrap().path())?;
        let buffered = BufReader::new(file);
        let mut data: Vec<String> = Vec::new();
        for line in buffered.lines() {
            data.push(line?);
        }
        chain.push(data);
    }
    Ok(chain)
}

pub fn read_data(name: String) -> Result<Vec<String>, Error> {
    let current_dir = &env::current_dir().unwrap();
    let base_dir = Path::new(BASIC_STORAGE_DIR_NAME);
    let name = Path::new(&name);
    let chain_data = env::join_paths([current_dir, base_dir, name].iter()).unwrap();
    log::info!("Read from: {:?}", chain_data);
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

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    #[test]
    fn test_write_data() {
        init_logger();
        let name = "000001";
        let data = "Test data";
        write_data(name.to_owned(), data).unwrap();
    }

    #[test]
    fn test_read_data() {
        init_logger();
        let name = "000001";
        let data = "Test data";
        let mut result = read_data(name.to_owned()).unwrap();
        assert_eq!(data.to_owned(), result.pop().unwrap());
    }
}
