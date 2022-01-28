use std::fs;

pub mod digit_decoder;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
}
