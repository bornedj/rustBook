use std::fmt::Display;
// every reference in Rust has a lifetime which is the scope for which the reference is valid
// similar to types they are inferred, and only need to be specified when references lifetimes could be related in multiple different ways
pub fn lifetimes() {
    // the following code will not compile
    // {
    //     let r;

    //     {
    //         let x = 5;
    //         r = &x;
    //     }// here the x value is dropped out of scope

    //     // we are creating a dangling references as x has dropped from scope and we are trying to refer to it in the println below
    //     println!("r: {}", r);
    // }

    // fixed version with scope annotated
    {
        let x = 5;       // ----------+-- 'b
                              //           |
        let r = &x;     // --+-- 'a  |
                              //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    }                         // ----------+

    // comparing string length
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // lifetimes don't change the length a reference lives, just how they relate to one another
    // &i32        // a reference
    // &'a i32     // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime

    // displaying how a shared generic lifetime becomes the shortest lifetime of the parameters
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }// 'a becomes the lifetime of string2 as is only valid during the inner scope

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// the function below will cause an error as the compiler doesn't know if the returning reference is of x or y
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// using lifetimes to correct the previous longest function
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// if you want to add references to structs you need to specify a lifetime with each reference
struct ImportantExcerpt<'a> {
    part: &'a str,
}


// lifetimes that apply to method parameters are input lifetimes, and lifetimes that apply to signatures are output lifetimes
// the compiler uses three rules to assign lifetimes that aren't explicitly given, the first applies to input lifetimes and the second and third applies to output lifetimes
// the compiler automatically assigns a lifetime for each input parameter
// rule 2: if there are exactly 1 input lifetime parameter, that lifetime is applied to all ouput lifetimes
// rule 3: if there are more than one input lifetime parameters, and on is &self or &mut self the lifetime of self is applied to all output lifetimes
// if all  three rules are worked through and we do not have a clear input and output lifetime, we need to explicitly state them

// the 'static lifetime is used for references that last the entire duration of your programs lifetime
// used for all string literals since they are written to the programs binary

// defining a function with all three, generic type parameters, trait bounds, and lifetimes.
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}