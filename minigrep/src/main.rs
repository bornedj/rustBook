#![allow(dead_code)]
pub use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();// return the args as vector of strings

    // creating references to our args
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // reading contents
    let contents = fs::read_to_string(config.filename).expect("Something went wrong, while reading the file");
    println!("With text:\n{}", contents);
}

// CLI parsing
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();
    Config {query, filename}
}

struct Config {
    query: String,
    filename: String
}