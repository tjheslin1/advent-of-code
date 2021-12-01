use std::fs;

pub mod depth;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let readings: Vec<u32> = contents
        .split("\n")
        .filter(|line| line.is_empty() == false)
        .map(|line| {
            line.parse::<u32>()
                .expect("Error parsing input, expected a number.")
        })
        .collect();

    let increases = depth::calculate_increased_depths(&readings);
    let three_measurement_increases =
        depth::calculate_three_measurement_increased_depths(&readings);

    println!("Increases = {}", increases);
    println!(
        "Three measurement increases = {}",
        three_measurement_increases
    );
}
