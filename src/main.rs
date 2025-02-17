use std::fs;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <query> <filename>", args[0]);
        process::exit(1);
    }

    let filename = &args[2];
    println!("In file {}", filename);

    match fs::read_to_string(filename) {
        Ok(contents) => println!("File content:\n{}", contents),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
