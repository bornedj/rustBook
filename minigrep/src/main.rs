use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();// return the args as vector of strings

    // creating references to our args
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);

    // reading contents
    let contents = fs::read_to_string(filename).expect("Something went wrong, while reading the file");
    println!("With text:\n{}", contents);
}
