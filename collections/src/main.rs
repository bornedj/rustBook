#![allow(unused_variables, dead_code)]

pub mod vectors;
pub mod strings;

use crate::vectors::SpreadsheetCell;

fn main() {
    vectors::vectors();

    //enum and vec together
    let row: Vec<SpreadsheetCell> = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    strings::strings();
}
