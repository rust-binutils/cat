use std::fs::File;
use std::{
    io::{BufReader, Read},
    path::Path,
};

pub fn read_file<T: AsRef<Path>>(path: T) -> anyhow::Result<Vec<u8>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
