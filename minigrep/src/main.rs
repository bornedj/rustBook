use std::{env, process};
use minigrep::Config;

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
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
