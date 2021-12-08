use std::fs;

pub mod bingo;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let (bingo_numbers, mut cards) = bingo::parse_bingo_input(&contents[..]);

    let result = bingo::run_bingo(bingo_numbers, &mut cards);
    println!("results = {}", result);

    let last_winning = bingo::determine_last_winning(bingo_numbers, &mut cards);
    println!("last_winning = {}", last_winning);
}
