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


pub mod ref_cell {
    pub trait Messenger {
        fn send(&self, msg: &str);
    }

    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }
}

pub mod tree {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};


    #[derive(Debug)]
    pub struct Node {
        pub value: i32,
        pub children: RefCell<Vec<Rc<Node>>>,
        pub parent: RefCell<Weak<Node>>,
    }
}

#[cfg(test)]
mod tests {
    use super::ref_cell::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    // below implementation fails as we need to follow interior mutabiliy pattern here
    // impl Messenger for MockMessenger {
    //     fn send(&self, message: &str) {
    //         self.sent_messages.push(String::from(message));
    //     }
    // }

    // using a refcell we can accomplish this
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}