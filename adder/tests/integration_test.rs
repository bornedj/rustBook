use adder;
mod utils;

// integration tests go within the test directory which is at the same level as the src directory
#[test] 
fn it_adds_two() {
    utils::setup();// example of using a subdirectory function
    assert_eq!(4, adder::add_two(2));
}

// you can use cargo test --test integration_filename to run all the integration tests in a file
