pub use std::collections::HashMap;

pub fn hash_maps() {
    // this type annotation is okay because rust can infer the type within vectors
    let mut scores1: HashMap<_, _> = HashMap::new();

    scores1.insert(String::from("Blue"), 10);
    scores1.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores2: HashMap<_,_> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    assert_eq!(scores1, scores2);

    // values of types with the copy trait like i32, their values are copied onto the has map
    // demonstration that owned values are the opposite
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    
    // we can access a value by providing its key to the get method
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    // program has to be able to error handle get to hash map because get returns an option

    // we can iterate over score with a for loop as well
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }


    // Overwritting a value a key has using insert 
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    //only writing if there is no value present at that key 
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // entry returns a an enum type Entry similar to an Option
    scores.entry(String::from("Blue")).or_insert(50);
    dbg!(scores);
    // The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists,
    // and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value. 
    // This technique is much cleaner than writing the logic ourselves and, in addition, plays more nicely with the borrow checker.

    // interacting with the old value in a key after lookup
    // example counting the number of words in a string based on white space
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}