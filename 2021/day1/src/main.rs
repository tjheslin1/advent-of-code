// use crate::depth;

pub mod depth;

fn main() {
    let increases = depth::calculate_increased_depths(vec![]);

    println!("Increases = {}", increases)
}
