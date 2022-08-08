#![allow(unused_variables, dead_code)]
pub mod panic;
pub mod recoverable_errors;

fn main() {
    panic::panic();
    recoverable_errors::recoverable_errors();
}
