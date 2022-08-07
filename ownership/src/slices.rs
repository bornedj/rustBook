//------------------------------------------------------------------------------
pub mod slices {
    // attempting to solve the problem without the use of slices
    pub fn first_word_no_slices(s: &String) -> usize { //returning the index of the first space
        let bytes = s.as_bytes(); // converting the string into an array of bytes to look at each element   

        // creating an iterator to go over the array o bytes
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }

    // slices are created with [] and range formating
    pub fn slices() {
        let s = String::from("hello world");

        // slice examples
        let hello = &s[0..5];
        let world: &str = &s[6..11];
        // [starting_index..ending_index] with ending_index non inclusive
        // slices truly stores the starting position and length which is the ending_index - startingindex
        // general note you can use the range syntax with ..5 == 0..5
        // block below displays that the range syntax allows you to drop the ending index if it is the last;
        let s = String::from("hello");
        let len = s.len();
        let slice_1 = &s[3..len];
        let slice_2 = &s[3..];
        assert_eq!(slice_1, slice_2);

        // the same goes for the full length
        let slice = &s[0..len];
        let slice = &s[..];
        assert_eq!(slice_1, slice_2)

    }

    // fixing first word now that we know how string slices working
    pub fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // other slices
    // you can slice arrays
    pub fn array_slice () {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}