#![allow(unused_variables, dead_code)]
pub mod extraction;
pub mod generics;
pub mod traits;
pub mod lifetimes;

use traits::{Summary, Tweet};

fn main() {
    extraction::extraction();
    generics::generics();

    // making use of the Summary trait and Tweet Struct
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
