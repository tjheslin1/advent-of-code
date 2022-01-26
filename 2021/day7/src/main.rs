#![feature(int_abs_diff)]

use std::fs;

pub mod crabmarine;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let (position, cost) =
        crabmarine::calculate_cheapest_horizontal_position(&input, crabmarine::CONSTANT_FUEL_RATE);

    println!("Optimal position = {}, with fuel cost = {}", position, cost)
}
