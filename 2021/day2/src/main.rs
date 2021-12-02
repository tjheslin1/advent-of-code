use std::fs;

pub mod travel;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let directions: Vec<(&str, &str)> = contents
        .split("\n")
        .filter(|line| line.is_empty() == false)
        .map(|line| {
            match &line.split(' ').collect::<Vec<&str>>()[..] {
                [command, distance] => (*command, *distance),
                e => panic!("Unexpected input of length: {:?}", e),
            }
            // line.parse::<u32>()
            // .expect("Error parsing input, expected a number.")
        })
        .collect();

    let coords = travel::determine_travel(&directions);

    println!("Coords: {}, {}", coords.0, coords.1);
}
