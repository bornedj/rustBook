fn main() {
    println!("Hello, world!");
    another_function(7);
    expression_example();
}

// example of a function that has a parameter 
fn another_function(x: u8) {
    println!("The value of x is: {x}");
}

// statements are lines of code within a function that do something, expressions are lines of code that return or result in something
// adding a ; to an expression turns it into a statement
fn expression_example() {
    // the inner braces are an expression that also contain a statement

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}