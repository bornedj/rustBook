// you can change the type ending the program has when panicking. aborting makes the OS clear the memory, while the default is to walk up the stack through functions unwinding
// aborting reducing program size, but can increase the computational effort to end the program
// [profile.release]
// panic = 'abort'

pub fn panic() {
    // calling panic to get panic
    // panic!("crash and burn");


    // getting panic from a library
    let v = vec![1,2,3];
    // v[99]; would produce index out of bounds error and panicked
    // we can change rust_backtrace env variable to see the backtrace that caused the error 
    // RUST_BACKTRACE=1 cargo run
    println!("Panic Finished")
}