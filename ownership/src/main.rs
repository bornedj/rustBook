#![allow(unused_variables, dead_code)]

fn main() {
    // example of scope for variable lifetime
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    // string literals like the previous variable s are good for predetermined values. Strings are used when you need mutable data type, or one with unknown size
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`


    // demonstrating rust's solution to the invalidated reference when pointing to the same location on the heap 
    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1);

    // displaying the clone method to complete a deep copy or copy of data on the heap
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //borrowing errors are caused when that variable is stored on the heap with an unknownsize at compile time. So copying variables on the stack like integers does not validate the rule
    // data types that have a known size on compile time and are stored on the stack can have the copy trait which allows the above comment ^^

    // example explaining how passing a variable to a function is the same as assigning it to another variable.

    //function calls to additional book unit portions
    passing_to_function_example();
    passing_through_function_return();
    passing_multiple_with_tuple();
    passing_through_function_return();
    reference_example();
    // below line would call function to create a dangling reference
    // let ref_to_nothing = dangle();
}

fn passing_to_function_example() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn passing_through_function_return() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// example of returning multiple values with a tuple
fn passing_multiple_with_tuple() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// ----------------------------------------------------------------------------
// references
fn reference_example() {
    let s1 = String::from("hello");

    let len = calculate_length_2(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

// attempting to create a dangling reference will not allow the program to compile
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// major references rules:
// At any given time, you can have either one mutable reference or any number of immutable references.
//References must always be valid.