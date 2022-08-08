pub fn extraction() {
    dup_no_generics();

    // using the largest function to reduce duplication
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    println!("Extraction finished\n");
}


// removing duplication without generics
// below function could get the largest number, but would need to be repeated to be used again
fn dup_no_generics() {
    let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }

    println!("The largest number is {}", largest);
}

// abstracting the largest function to reduce duplicates
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}