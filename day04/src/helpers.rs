use anyhow::{Result, Context};
use std::{path::Path, fs::File, io::{BufReader, BufRead}};

pub fn parse_file<P: AsRef<Path>>(fpath: P) -> Result<impl Iterator<Item = String>> {
    // file path
    let path = fpath.as_ref();
    // create a File
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: '{}'", path.display()))?;
    // create a BufReader around File
    let reader = BufReader::new(file)
        .lines()
        .take_while(|line| line.is_ok())
        .map(|line| line.unwrap());
    // return an iterator over each line
    Ok(reader)
}