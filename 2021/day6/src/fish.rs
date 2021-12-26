use std::collections::HashMap;

const DAYS_TO_CREATE_FISH: u32 = 6;
const NEW_FISH_EXTRA_DAYS: u32 = 2;

pub fn simulate_spawn(input: &str, days_to_simulate: usize) -> usize {
    let mut fish_school = input
        .replace(" ", "")
        .split('\n')
        .filter(|line| *line != "")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    (0..days_to_simulate).for_each(|_| {
        let mut spawned_fish: Vec<u32> = vec![];
        fish_school.iter_mut().for_each(|fish| {
            if *fish == 0 {
                *fish = DAYS_TO_CREATE_FISH;
                spawned_fish.push(DAYS_TO_CREATE_FISH + NEW_FISH_EXTRA_DAYS);
            } else {
                *fish -= 1;
            }
        });

        fish_school.append(&mut spawned_fish);
    });

    fish_school.len()
}

fn parse_input(input: &str) -> HashMap<usize, u32> {
    let fish_school = input
        .replace(" ", "")
        .split('\n')
        .filter(|line| *line != "")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut spawned_fish: HashMap<usize, u32> = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);

    fish_school.iter().for_each(|fish| {
        if let Some(curr_count) = spawned_fish.get(fish) {
            spawned_fish.insert(*fish, curr_count + 1);
        } else {
            panic!("Unexpected value for fish age: {}", fish);
        }
    });

    spawned_fish
}

#[cfg(test)]
mod tests {

    use crate::fish::*;

    #[test]
    fn test_input_example() {
        let example = "3,4,3,1,2";

        let actual_num_fish_after_18_days = simulate_spawn(example, 18);
        assert_eq!(actual_num_fish_after_18_days, 26);

        let actual_num_fish_after_80_days = simulate_spawn(example, 80);
        assert_eq!(actual_num_fish_after_80_days, 5934);
    }

    #[test]
    fn test_live_forever() {
        let example = "3,4,3,1,2";

        let actual_num_fish_after_256_days = simulate_spawn(example, 256);
        assert_eq!(actual_num_fish_after_256_days, 26984457539);
    }

    #[test]
    fn test_parse_input() {
        let example = "3,4,3,1,2";

        let actual_parsed_input = parse_input(example);

        let expected_parsed_input = HashMap::from([
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 1),
            (5, 0),
            (6, 0),
            (7, 0),
            (8, 0),
        ]);

        assert_eq!(actual_parsed_input, expected_parsed_input);
    }
}
