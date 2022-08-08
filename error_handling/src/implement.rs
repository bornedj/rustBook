use core::panic;
use std::net::IpAddr;

pub fn implement() {
    // if you know something won't fail it's okay to leave the unwrap call
    let home: IpAddr = "127.0.0.1".parse().unwrap();// I know parse won't fail since I'm converting a string literal to the type
    
    // it's important to use panic/unrecoverable when your program has entered a bad state
    // a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
        // The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
        // Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
        // There’s not a good way to encode this information in the types you use. We’ll work through an example of what we mean in the “Encoding States and Behavior as Types” section of Chapter 17.

    // when failures are expected you should return the result type 
    println!("implement finished");
}

// creating custom types for validation
// example going back to the guessing game, creating a type to ensure  the user has entered a valid number
pub struct Guess {
    value: i32,
}

// within the new function we are handling the error 
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess {value}
    }
    // need a getter as the field is private
    pub fn value(&self) -> i32 { 
         self.value
    }
}
// any time we need to return a number between 1 and 100 we can use the Guess struct