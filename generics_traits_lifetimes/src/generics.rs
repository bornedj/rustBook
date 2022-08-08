pub fn generics() {
    // calling the signed int version
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    // calling char version
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // using the generic function
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // end calls
    println!("Generics Finished\n");

    //portion on monomorphization
    monomorphization();
}


// going to use generics to combine the extraction functions below
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// We read this definition as: the function largest is generic over some type T. This function has one parameter named list, which is a slice of values of type T. The largest function will return a value of the same type T.
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest 

// }
// code won't compile because the generic T does not have the correct trait to allow binary comparison

//fixed largest
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0]; 
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}


// you can also use generics in structs
// since only one generic is given, x and y must be the same type
struct Point<T> {
    x: T,
    y: T
}
// here a Point can have x and y coordinates of any type


// you can also use generics with enums
// looking at the provided option enum
enum Option<T> {
    Some(T),
    None,
}

// result is an enum with multiple generics
enum Result<T, E> {
    Ok(T),
    Err(E)
}

// you can add methods and associated functions with generics
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


// what the enums of options look like at runtime
enum OptionI32 {
    Some(i32),
    None,
}

enum OptionF64 {
    Some(f64),
    None,
}

// monomorphization prevents runtime performance decreases due to generics by converting generic types to specified types used in compilation
fn monomorphization() {
    let integer = Some(5);
    let float = Some(5.0);

    let integer = OptionI32::Some(5);
    let float = OptionF64::Some(5.0);
    println!("Monomorphization finished\n");
}

