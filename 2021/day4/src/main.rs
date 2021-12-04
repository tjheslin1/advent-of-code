use std::fs;

pub mod bingo;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    println!("Hello, world!");
}
