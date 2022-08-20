#![allow(dead_code)]
use oop::gui::{Draw, Screen, Button};
fn main () {
    // trait objects
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            })
        ]
    };
    screen.run();

    // function below uses the oop state pattern accomplished through traits
    state_pattern();

    // encoding states and behaviors as types
    state_as_types();
}

fn state_pattern () {
    use oop::blog::{Post};
    // state pattern through blog post example
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}

fn state_as_types () {
    use oop::blog_types::Post;

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

}

// demonstrating that a user could add more gui components as long as they implement the Draw Trait
// implementation of the select box is similar to duck typing in dynamicly typed languages
#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("{self:?}");
    }
}