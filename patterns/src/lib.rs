// within the match the none and Some of the arms are the pattern
pub fn match_pattern () {
    let x = Some(5);
    match x {
        None => None,
        Some(i) => Some(i + 1),
    };
}

// the if let pattern also allows else if and else if let logic
pub fn if_let () {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

pub fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // will run as long as a pattern continues to match
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

// destructuring a tuple within a for loop
pub fn destructure_for () {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

// you can use inclusive ranges on char's based on ascii
pub fn range_with_char () {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

// shows how to destructure a struct
pub fn destruct_struct () {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // creates variables a, and b from x,y respectively from the point
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);


    // shorthand version
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

// demonstrating how match introduces a shallow copy
pub fn match_shallow() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
