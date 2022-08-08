use std::fs::File;
use std::io::ErrorKind;

pub fn recoverable_errors() {
// will through an error
// error_handling_with_match(); 
kind();
kind_methods();
// error_unwrap()
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