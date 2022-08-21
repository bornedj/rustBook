#![allow(dead_code)]
// below is a type alias
type Kilometers = i32;

pub fn type_alias () {
    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x+y);

}

// type alias are primarily used to reduce the amount of times you have to write a long type f.e.
type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn long_type_alias () {

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    // fn returns_long_type() -> Thunk {
    //     // --snip--
        
    // }
}


// the never type ! or empty type
// when used in a function signature, that function can never return something
// continue has the ! type
// fn foo () -> ! {
// }