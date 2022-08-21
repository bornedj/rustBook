use advanced_rust;
fn main () {
    advanced_rust::raw_pointers();// declaration of raw pointers outside of an unsafe block
    advanced_rust::raw_pointer_arbitrary(); // declares a raw pointer at an arbitrary memory location. Just a demonstration that this is possible, there are extremely few cases where you would chose to do this
    advanced_rust::deref_raw_pointer(); // uses an unsafe block to deref a raw pointer, and print the value 
    // dangerous does nothing, but is an unsafe function. Used to demonstrate that there has to be a unsafe block to call the function
    unsafe {
        advanced_rust::dangerous();
        println!("Absolute value of -3 according to C: {}", advanced_rust::abs(-3));// uses an external function with the extern key word
    }
    advanced_rust::using_split_at_mut(); // split_at_mut is a standard method of Vecs that splits a vector at a given index, and must be written using unsafe rust code
    // function must be written using unsafe rust as it makes to mutable borrows of a vector
    
    // my_split_at_mut is a demonstration of how the split_at_mut fn is implemented, asserts show it splits a mut ref to an array into two mut refs of the array by a given index
    let values = &mut [1,2,3,4];
    let (a, b) = advanced_rust::my_split_at_mut(values, 2);
    assert_eq!(a, &mut [1,2]);
    assert_eq!(b, &mut [3,4]);
}