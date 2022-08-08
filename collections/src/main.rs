#![allow(unused_variables, dead_code)]
pub mod vectors;
pub mod strings;
pub mod hash_maps;

use crate::vectors::SpreadsheetCell;

fn main() {
    // check the vectors mod
    vectors::vectors();

    //enum and vec together
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // check the strings mod
    strings::strings();
    // check the hash_maps mod
    hash_maps::hash_maps();
}
