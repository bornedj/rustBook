use std::ops::Deref;
use std::rc::Rc;
// implementing a cons list from the lisp language here, will cause error as the recursive type is not handled
// pub enum List {
//     Cons(i32, List),
//     Nil,
// }

// implementing a cons list from the lisp language here
// converted to a Reference counter for cons recursive type
// pub enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

pub enum List {
    Cons(i32, Rc<List>),
    Nil
}

// implementing my own box
pub struct MyBox<T>(T);

// had to add the deref trait below to allow myBox to behave the same as the std box
impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
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
pub fn hello(name: &str) {
    println!("Hello, {name}");
}

// struct for custom smart pointer
pub struct CustomSmartPointer {
    pub data: String,
}

// demonstrating the drop trait
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
