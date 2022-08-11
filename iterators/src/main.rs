fn main() {
    // example use case of iterator to print items in a vector
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("{v1:?}"); // showing that the vector was not consumed when the iterator was initialized
    for val in v1_iter { // iterator not consumed until here
        println!("Got: {}", val);
    }
}

// how the Iterator trait looks that the iterator type implements
pub trait Iterator {
    type Item;// associated type, will cover this later

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
