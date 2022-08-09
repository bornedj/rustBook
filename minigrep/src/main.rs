use std::{env, process};
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();// return the args as vector of strings

    // creating references to our args
    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // reading contents
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
