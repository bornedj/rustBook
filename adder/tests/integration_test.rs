use adder;

// integration tests go within the test directory which is at the same level as the src directory
#[test] 
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}