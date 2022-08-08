// traits define the functionality a specific type has
// similar to interfaces in other languages
// newsarticle and tweet example 
pub trait Summary {
    // fn summarize(&self) -> String;// all types with the Summary trait will have a method called summarize that takes a reference of itself and returns a string
    // it is up to the definition of the type to define what the summary method will do, but it must meet the signature and parameters

    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// below are the news article and tweet structures that will use the summary trait
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("Written by: {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
// allows you to call the summarize on the item parameter, which is some type that implements the Summary trait

// impl trait syntax is actually syntax sugar for the following, this is known as a trait bound
pub fn notify_non_impl<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// can also be used in the signature to make sure a type with an impl trait is returned
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}