pub fn vectors() {
    // creating a new empty vector
    let example_v: Vec<i32> = Vec::new();

    // creating a new vector using the vec! macro
    let mut v = vec![1, 2, 3, 4];
    // addign to a vector by pushing to the end of it
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);


    //demonstrating reading from a vector
    let third: &i32 = &v[2]; // using index
    println!("The third element is {}", third);

    match v.get(2) { //using the get method
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // get can return none without panicking which it's use case

    //iterating over each instance of a vector
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // iterating with mutable instances
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;// dereference to change the actual values within the vector
    }
    for i in &v {
        println!("{}", i);
    }
}

// enum to be used with vector, cells can have three data types
pub enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
