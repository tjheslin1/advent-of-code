use std::fs;

pub mod diagnostics;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let readings: Vec<&str> = contents
        .split('\n')
        .filter(|line| line.is_empty() == false)
        .collect();

    let (gamma, epsilon) = diagnostics::diagnostics_report(&readings);

    println!("gamma = {}, epsilon = {}", gamma, epsilon);
}
