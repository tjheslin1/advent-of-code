use std::fs;

pub mod vents;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let lines = vents::parse_input(&input[..]);
    let overlap_count = vents::find_overlapping_points(&lines[..], 2);

    println!("overlap_count = {}", overlap_count);
}
