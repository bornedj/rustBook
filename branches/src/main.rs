fn main() {
    let number = 6;

    //example if else statment
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // example of multiple branches
    // with number == 6, the divisible by 3 will display as it's the first block that evaluates to true
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // you can use if within declaration similar to ternary operator declarations in JS
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");


    //blocks of ifs or branches must all result in the same type, or the compiler with throw an error 

    //rust supports three loop types: loop, while, for
    // while takes a conditional, loop runs until explicity told to stop, and for loops run a set number of times
    // break can be used to exit a loop at any point, and continue can be used to skip over the remainder of the arm's code
    // example use for a loop
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
    }

    // demenstration of how break and continue are applied within nested loops
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // example while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // example for loop using a range 
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
