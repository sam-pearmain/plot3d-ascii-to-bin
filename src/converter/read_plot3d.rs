#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_plot3d(input_path: &str) -> io::Result<()> {
    if !input_path.to_lowercase().ends_with(".xyz") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "input file must have .xyz extension"
        ));
    }
    
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let mut lines = reader.lines();

    // read number of blocks and validate number of blocks
    let nblocks: i32 = lines
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "input file is empty"))??
        .trim()
        .parse()
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid number of blocks, file possible corrupt"))?;
    
    if nblocks <= 0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("invalid number of blocks: {0}", nblocks)
        ));
    }


    todo!();
}