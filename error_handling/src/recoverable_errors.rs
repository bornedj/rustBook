use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

pub fn recoverable_errors() {
// will through an error
// error_handling_with_match(); 
kind();
kind_methods();
// error_unwrap()
let username1 = read_username_from_file();
let username2 = read_username_operator();
let username3 = even_shorter_read_username();
let username4 = shortest_read_username();
println!(
    "Username or error 1: {:?}, Username or error 2: {:?}, Username or error 3: {:?}, Username or error 4: {:?}",
    username1,
    username2, 
    username3,
    username4
);
//should all be the same we are reading from the same file without editing it
println!("Recoverable_errors finished");
}


fn error_handling_with_match() {
    //File::open returns a result so we can expect a recoverable error
    let f = File::open("hello.txt");

    // error handling with match
    let f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => panic!("Problem opening the file: {:?}", error), 
    };
    // shows a notfound file error 
}

// we can handle an error by matching its variant with kind()
fn kind() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {// need the std::io::ErrorKind for this
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// different syntax with the methods around results to get same function as kind
// uses closures
fn kind_methods() {
  let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}


//you can also use unwrap
fn error_unwrap() {
    let f = File::open("hello.txt").unwrap();
}

//propagating the error aka passing the error back as a result
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// propagating is so common that rust provides the ? operator to make this easier
fn read_username_operator() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn even_shorter_read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn shortest_read_username() -> Result<String, io::Error> {
    //fs provides a function that creates a new string, opens a file and writes its contents into a string if possible
    fs::read_to_string("hello.txt")
}
// you can only implement the ? operator when the FromResidual trait is applied to the return type of the function