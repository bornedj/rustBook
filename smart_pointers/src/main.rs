#![allow(unused_variables)]
use List::{Cons, Nil};
fn main() {
    // example showing how to use a box, 5 is stored on the heap here
    let b = Box::new(5);
    println!("b = {}", b);

    // using the cons list we have created 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

// implementing a cons list from the lisp language here, will cause error as the recursive type is not handled
// pub enum List {
//     Cons(i32, List),
//     Nil,
// }

// implementing a cons list from the lisp language here
enum List {
    Cons(i32, Box<List>),
    Nil,
}