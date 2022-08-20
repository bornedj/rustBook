#![allow(unused_variables)]
use std::mem::drop;
use std::rc::Rc;
use smart_pointers::List::{Cons, Nil};

fn main() {
    // example showing how to use a box, 5 is stored on the heap here
    let b = Box::new(5);
    println!("b = {}", b);

    // using the cons list we have created 
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));


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


    // this will not compile as it demonstrates why we need reference counters Rc<t>
    // the value is moved into the list b and cannot be used in a
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // new version of list uses reference counters and the above block will work
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // calling Rc::clone instead of a.clone() as a.clone() creates a deep copy, while Rc::clone increments the reference count
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));


    // demonstrating how a reference counter handles items leaving scope
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}

