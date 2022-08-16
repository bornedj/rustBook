#![allow(unused_variables)]
use List::{Cons, Nil};
use std::ops::Deref;
fn main() {
    // example showing how to use a box, 5 is stored on the heap here
    let b = Box::new(5);
    println!("b = {}", b);

    // using the cons list we have created 
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


    // demonstrating the deref trait
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // showing that the deref operator works for boxes as they implement the trait
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // showing the deref on myBox
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // demonstrating deref coercion as the hello fn takes a &str arg
    let my_name = MyBox::new("Daniel".to_string());
    hello(&my_name);
    // call below demonstrates what the function call would look like without deref coercion
    hello(&(*&my_name)[..]);

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

// implementing my own box
struct MyBox<T>(T);

// had to add the deref trait below to allow myBox to behave the same as the std box
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// correct implementation of myBox, allows the deref method to be used on the object so the asserts main will pass
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// function used in deref coercion example
fn hello(name: &str) {
    println!("Hello, {name}");
}