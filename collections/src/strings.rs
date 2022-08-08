#![allow(unused_assignments)]//string declartion with new for example, never read before reassignment
pub fn strings() {
    //many of the methods/operations that are available to Vecs are applicable to Strings
    let mut example_s = String::new();// assigning a string with new this way can create an unused_assignments warning
    let data = "initial contents";
    example_s = data.to_string(); // function on data types with Display trait which includes string literals

    // can skip the first assigment by doing it on a literal
    let s = "initial contents".to_string();
    let s: String = String::from("initial contents"); // or this works too
    assert_eq!(example_s, s);

    // showing methods of string concatenation
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // demonstration that push_str() pushes a string literal through ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push takes a single char an adds it to a String
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // addition operator concat
    let s1 = String::from("Hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // using a reference to show that s1 will be dropped here
    // fn add(self, s: &str) -> String { reason the s1 is dropped is because the add function looks something like this
    // you can't add to strings together, compiler can coerce the &String to &str as it is a &str when String[..]
    // does drop s1 because it is self, more efficient than copying both strings
    println!("{s3}");

    // lots of concates are better with format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let string = s1 + "-" + &s2 + "-" + &s3;

    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let string2 = format!("{}-{}-{}", s4, s5, s6);
    assert_eq!(string,string2);

    //you cannot access individual characters of a string with indexing syntax in Rust
    // this is because characters can be bigger than a bytes than others, and that is how a strings lenght/indexing is done

    // you need to iterate through strings as slicing them is risky 
    // method for looking the chars which are close to graphemes which are like letters
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // also can show the bytes of a string
    for b in "नमस्ते".bytes() {
        println!("{b}");
    }

    // there are crates to to get it grapheme if needed.
    // testing how close plain english characters are as chars
    for c in "testing".chars() {
        println!("{c}");
    }
}