#![allow(unused_variables, dead_code)]

//enums are a custom data type that use variants
// enums can only be one kind of their variant
// useful for IP addresses for example
enum IpAddrKind {
    V4,
    V6
}

fn main() {
    // creating instances of enums
   let four: IpAddrKind = IpAddrKind::V4;
   let six: IpAddrKind = IpAddrKind::V6;
   // both 4 and six are type IpAddrKind which is part of the functionality of enums
    
   route(four);
   route(six);


    // way to store more information around IP address with struct
    let home = StructIpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = StructIpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // enum way to store string IP address with type
    // enums with data also act as an automatic constructor function
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // enums with different types
    let home = IpAddrDiffTypes::V4(127, 0, 0, 1);
    let loopback = IpAddrDiffTypes::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    // optins are enums that are in the prelude and allow you to deal with the concept of null
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // useful as options force a type difference

    // match control flow with enums
    let nickel = Coin::Nickel;
    print!("value of nickel: {}", value_in_cents(nickel));

    // using match function with options
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    
    // using match with functions on an enum
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

// function to demonstrate shared type
fn route(ip_kind: IpAddrKind) {}

// way to store more information around IP address with struct
struct StructIpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddrDiffTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum with multiple different types
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    // Quit has no data associated with it at all.
    // Move has named fields like a struct does.
    // Write includes a single String.
    // ChangeColor includes three i32 values.
}

// you can also write methods on enums
impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
    
}

// match control flow construct
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// using match with options
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

// you can also use match with functions to use logic on enums
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}