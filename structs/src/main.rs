#![allow(unused_variables, dead_code)]
mod tuple_structs;

use tuple_structs::{Color, Point};

fn main() {
    //initializing user
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1);

    // function to build user
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    // shorthand for whne field name is the same as the variable name
    fn build_user_shorthand(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // using the struct update syntax to create a user based on another
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // same as above block but uses the spread operator
    // spread must come last in the declaration
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    
    //tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // they behave like tuples just provide a typing to them essentially 
    // can be destructured and accessed with .idx syntax


    // you can also have unit-like structs useful for applying traits without storing data
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // it is possible to store references in structs but we will get to that in a later chapter on lifetimes


}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
