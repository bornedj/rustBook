#![allow(unused_variables, dead_code)]
// code from chapter 5 on structs importing this way  for example
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // first example test
    #[test]
    fn exploration() {
        assert_eq!(3+2, 5);
    }

    // failing tests occur when something panics
    #[test]
    #[should_panic]// can add the should panic attribute to run tests that are intended to panic
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    #[should_panic]
    fn smaller_can_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(smaller.can_hold(&larger));
    }

    //showing the assert eq method
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // you will need to implement the partialEq and Debug traits to enums in order to use the assert_eq

    // demonstrating how you can use formating with error message for the assert functions
    #[test]
    #[should_panic]// remove should panic to demonstrate
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // you can write tests that return enums
    #[test]
    fn it_works_enum() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

// calling cargo test with -- has options for the binary
// cargo test --show-output will show the stdout for tests that pass
// cargo test --test-threads=1 will use only one thread and run all tests in order instead of parallel, useful for when tests depend on a shared state
// cargo test fn_name will only run the specified function/test
// cargo test partial_function_name tests with names that match the given partial
// the ignore attribute can be added to tests that want to be ignored on the cargo test call
// can run only ignored tests with cargo test -- --ignored
//can run all tests including ignored with cargo test -- --include-ignored


//convetion to keep unit tests in the src directory in the file they are 
// integration tests are external from the library and interface with your libraries public api