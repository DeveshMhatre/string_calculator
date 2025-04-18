use std::io::{self, BufRead};

use string_calculator::add;

fn main() {
    println!("Expression (end with an empty line):");

    let stdin = io::stdin();
    let mut input = String::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        input.push_str(&line);
        input.push('\n');
    }

    if input.ends_with('\n') {
        input.pop();
    }

    println!("Answer: {}", add(input));
}
