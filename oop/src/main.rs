#![allow(dead_code)]
use oop::{Draw, Screen, Button};
fn main () {
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