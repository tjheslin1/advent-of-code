use std::fs;

pub mod fish;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let num_fish = fish::simulate_spawn(&input[..], 80);
    println!("Num fish after 80 days = {}", num_fish);
}
