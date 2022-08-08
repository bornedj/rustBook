pub fn strings() {
    //many of the methods/operations that are available to Vecs are applicable to Strings
    let mut example_s = String::new();// assigning a string with new this way can create an unused_assignments warning
    let data = "initial contents";
    example_s = data.to_string(); // function on data types with Display trait which includes string literals

    // can skip the first assigment by doing it on a literal
    let s = "initial contents".to_string();
    let s: String = String::from("initial contents"); // or this works too
    assert_eq!(example_s, s);
}