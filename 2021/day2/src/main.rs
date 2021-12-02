use std::fs;

pub mod travel;

fn main() {
    let contents: String =
        fs::read_to_string("input.txt").expect("Something went wrong reading the file");

    let directions: Vec<(&str, u32)> = contents
        .split("\n")
        .filter(|line| line.is_empty() == false)
        .map(|line| {
            match &line.split(' ').collect::<Vec<&str>>()[..] {
                [command, distance] => {
                    let parsed_distance: u32 = distance
                        .parse::<u32>()
                        .expect("Error parsing input, expected a number.");

                    (*command, parsed_distance)
                }
                e => panic!("Unexpected input of length: {:?}", e),
            }
            // line.parse::<u32>()
            // .expect("Error parsing input, expected a number.")
        })
        .collect();

    let coords = travel::determine_travel(&directions);
    let aimed_coords = travel::determine_travel_with_aim(&directions);

    println!("Coords: {}, {}", coords.0, coords.1);
    println!("Coords: {}, {}", aimed_coords.0, aimed_coords.1);
}
