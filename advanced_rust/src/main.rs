use advanced_rust::unsafe_rust;
use advanced_rust::advanced_traits::{Point, Human, Pilot, Wizard, Dog, Animal, OutlinePrint, Wrapper};
use advanced_rust::advanced_types;
fn main () {
    unsafe_rust::raw_pointers();// declaration of raw pointers outside of an unsafe block
    unsafe_rust::raw_pointer_arbitrary(); // declares a raw pointer at an arbitrary memory location. Just a demonstration that this is possible, there are extremely few cases where you would chose to do this
    unsafe_rust::deref_raw_pointer(); // uses an unsafe block to deref a raw pointer, and print the value 
    // dangerous does nothing, but is an unsafe function. Used to demonstrate that there has to be a unsafe block to call the function
    unsafe {
        unsafe_rust::dangerous();
        println!("Absolute value of -3 according to C: {}", unsafe_rust::abs(-3));// uses an external function with the extern key word
    }
    unsafe_rust::using_split_at_mut(); // split_at_mut is a standard method of Vecs that splits a vector at a given index, and must be written using unsafe rust code
    // function must be written using unsafe rust as it makes to mutable borrows of a vector
    
    // my_split_at_mut is a demonstration of how the split_at_mut fn is implemented, asserts show it splits a mut ref to an array into two mut refs of the array by a given index
    let values = &mut [1,2,3,4];
    let (a, b) = unsafe_rust::my_split_at_mut(values, 2);
    assert_eq!(a, &mut [1,2]);
    assert_eq!(b, &mut [3,4]);

    // overriding a standard operator to allow us to add to custom structs together
     assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // human implements multiple traits each with a fly method, we need to use the fish syntax to use a specific method or rust will default to the one implemented on the struct itself
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());// will use the baby_name method from dog struct as rust cannot infer to use Animal associated function
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());// syntax needed to use the Animal associated function on a Dog
    // above is using the "fully qualified syntax"

    // Demonstrating a supertrait with outline_print which is dependent on a type implementing fmt::Display
    let print_point = Point {x: 1, y: 2};
    print_point.outline_print();

    // demonstrating the newtype pattern
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // shows how to create a type alias with the kilometers type
    advanced_types::type_alias();
    advanced_types::long_type_alias(); // shows common use for type alias, reducing the number times you write a long signature or parameter
}