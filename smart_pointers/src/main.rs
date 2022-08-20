#![allow(unused_variables)]
use std::mem::drop;
use smart_pointers::List::{Cons, Nil};
use smart_pointers;

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
    let y = smart_pointers::MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // demonstrating deref coercion as the hello fn takes a &str arg
    let my_name = smart_pointers::MyBox::new("Daniel".to_string());
    smart_pointers::hello(&my_name);
    // call below demonstrates what the function call would look like without deref coercion
    smart_pointers::hello(&(*&my_name)[..]);

    // demonstrating how an object is dropped with a defined drop trait
    let c = smart_pointers::CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = smart_pointers::CustomSmartPointer {
        data: String::from("other stuff"),
    };
    // c.drop(); // would cause compile error, you can't use the drop method manually have to use std::mem::drop
    drop(c); // shows that the implemenation of the drop trait works with the mem::drop
    println!("CustomSmartPointers created.");

}
