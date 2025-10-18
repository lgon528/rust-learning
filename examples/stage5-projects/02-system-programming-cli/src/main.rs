use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|err| {
        eprintln!("Error opening file: {}", err);
        process::exit(1);
    });

    let reader = io::BufReader::new(file);
    let line_count = reader.lines().count();

    println!("Number of lines in {}: {}", filename, line_count);
}
