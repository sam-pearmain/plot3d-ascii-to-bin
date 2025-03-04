mod plot3d;

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("usage: {} <input_ascii_file> <output_binary_file>", args[0]);
        process::exit(1);
    }

    let input_file = &args[1];
    let output_file = &args[2];

    match plot3d::convert_plot3d_ascii_to_binary(input_file, output_file) {
        Ok(_) => {
            println!("successfully converted {} to {}", input_file, output_file);
        }
        Err(e) => {
            eprintln!("error converting file: {}", e);
            process::exit(1);
        }
    }
}
