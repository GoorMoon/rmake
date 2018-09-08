#[macro_use]
extern crate nom;
pub mod parser;
pub mod validators;

use std::str::Lines;

pub fn collapse_lines(iter: &mut Lines) -> Option<String> {
    let mut new_str = String::new();

    loop {
        if let Some(new_line) = iter.next() {
            new_str.push_str(new_line);
            if !new_line.ends_with("\\") {
                break;
            }
            new_str = new_str[..new_str.len() - 1].to_string();
        } else {
            return None;
        }
    }
    Some(new_str)
}
