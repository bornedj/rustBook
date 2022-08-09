// #![allow(dead_code)]
pub use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();// return the args as vector of strings

    // creating references to our args
    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // reading contents
    run(config);
}

// CLI parsing
// depreciated because of the config constructor
// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config {query, filename}
// }

struct Config {
    query: String,
    filename: String
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone(); 
        let filename = args[2].clone(); 
        Ok(Config {query, filename})
    }
}

// function takes the config and reads the file with the appropriate name
fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}