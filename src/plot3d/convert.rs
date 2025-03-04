#![allow(dead_code)]

use std::io;
use super::read::read_plot3d_ascii;
use super::write::write_plot3d_binary;

pub fn convert_plot3d_ascii_to_binary(input: &str, output: &str) -> io::Result<()> {
    // read the ascii plot3d file
    let blocks = read_plot3d_ascii(input)?;

    // write the binary plot3d file (using double precision)
    write_plot3d_binary(output, &blocks, true)?;

    Ok(())
}