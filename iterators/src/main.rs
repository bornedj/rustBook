#![allow(dead_code)]
fn main() {
    // example use case of iterator to print items in a vector
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    println!("{v1:?}"); // showing that the vector was not consumed when the iterator was initialized
    for val in v1_iter { // iterator not consumed until here
        println!("Got: {}", val);
    }


    //showing an iterator adapter that does not consume the iterator. Here you have to call collect method and assign the value since map does not consume an iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// how the Iterator trait looks that the iterator type implements
pub trait Iterator {
    type Item;// associated type, will cover this later

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

// shoe will be used in filter example to demonstrate the 
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // test showing how the filter iterator adapter takes in an environment using closures
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        // test showing the use of the next method on iterators
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    // displaying that sum consumes an iterator making it a consuming adaptor
    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();

        assert_eq!(total, 6);
    }
}