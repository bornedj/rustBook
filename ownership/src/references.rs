pub mod references {
    // ----------------------------------------------------------------------------
    // references
    pub fn reference_example() {
        let s1 = String::from("hello");

        let len = calculate_length_2(&s1);

        println!("The length of '{}' is {}.", s1, len);
    }

    pub fn calculate_length_2(s: &String) -> usize {
        s.len()
    }
}

