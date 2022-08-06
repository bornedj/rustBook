#![allow(unused_variables)]
use std::io;

fn main() {
    // block on mutability
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value x is: {x}");
    
    // block on variable scope and shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}")
    }
    println!("The value of y in the outer scope is: {y}");

    // general note on data sizes.
    // for signed integers (2^n-1) - 1 and -(2^n-1) - 1 for the limit on the number size
    // for unsigned integers it's (2^n) - 1

    // when scalar overflow occurs in a release version of the exe, the variable wraps around. So for a u8 with value of 256, the program would wrap to 0, then 1 for 257

    //block displaying supported mathematical operators
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    //compound types
    //Rust has two primitive compound types, tuples and arrays.
    //Tuples cannot change length after declaration. Their values can be accessed through destructuring or .Index notation
    // units are empty tuples, which is the default return type of expressions and functions
    // arrays are essentially tuples that must be the same data type, they also need a fixed length. Stored on the stack rather the heap unlike tuples
    // vectors are flexible arrays that can change in length
    
    // example of workign with array indexing
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");


}
