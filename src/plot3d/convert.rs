#![allow(dead_code)]

use std::io;
use super::read::read_plot3d_ascii;
use super::write::write_plot3d_binary;

pub fn convert_plot3d_ascii_to_binary(input: &str, output: &str) -> io::Result<()> {
    // read the ascii plot3d file
    let blocks = read_plot3d_ascii(input)?;

    // print info about the blocks
    println!("found {} blocks", blocks.len());
    for (i, block) in blocks.iter().enumerate() {
        println!(" - block {}: dimensions {} x {} x {}",
            i + 1,
            block.imax,
            block.jmax,
            block.kmax,
        );
    }

    // write the binary plot3d file (using double precision)
    write_plot3d_binary(output, &blocks, true, false)?;

    Ok(())
}