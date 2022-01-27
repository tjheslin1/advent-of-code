pub const CONSTANT_FUEL_RATE: &dyn Fn(usize, usize) -> usize = &|a: usize, b: usize| a.abs_diff(b);

pub const INCREASING_FUEL_RATE: &dyn Fn(usize, usize) -> usize = &|a: usize, b: usize| {
    let distance = a.abs_diff(b);

    (0..distance).fold(0, |acc, step| acc + 1 + step)
};

pub fn calculate_cheapest_horizontal_position(
    input: &str,
    fuel_cost_calculator: &dyn Fn(usize, usize) -> usize,
) -> (usize, usize) {
    let crabmarine_positions = input
        .replace(" ", "")
        .split('\n')
        .filter(|line| *line != "")
        .collect::<Vec<&str>>()
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    let max_position = crabmarine_positions
        .iter()
        .fold(
            crabmarine_positions[0],
            |acc, pos| if *pos > acc { *pos } else { acc },
        );

    (0..max_position).fold((0, usize::max_value()), |(acc_pos, acc_cost), position| {
        let fuel_cost =
            calculate_fuel_cost_for_move(position, &crabmarine_positions, fuel_cost_calculator);

        if fuel_cost < acc_cost {
            (position, fuel_cost)
        } else {
            (acc_pos, acc_cost)
        }
    })
}

fn calculate_fuel_cost_for_move(
    desired_position: usize,
    crabmarine_positions: &Vec<usize>,
    fuel_cost_calculator: &dyn Fn(usize, usize) -> usize,
) -> usize {
    crabmarine_positions.iter().fold(0, |acc, crab_pos| {
        acc + fuel_cost_calculator(*crab_pos, desired_position)
    })
}

#[cfg(test)]
mod tests {

    use crate::crabmarine::*;

    #[test]
    fn test_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let (actual_pos, actual_cost) =
            calculate_cheapest_horizontal_position(input, CONSTANT_FUEL_RATE);

        assert_eq!(actual_pos, 2);
        assert_eq!(actual_cost, 37);
    }

    #[test]
    fn test_example_incrementing_fuel_cost() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let (actual_pos, actual_cost) =
            calculate_cheapest_horizontal_position(input, INCREASING_FUEL_RATE);

        assert_eq!(actual_pos, 5);
        assert_eq!(actual_cost, 168);
    }

    #[test]
    fn test_calculate_zero_fuel_cost() {
        let input = vec![1];

        let actual = calculate_fuel_cost_for_move(1, &input, CONSTANT_FUEL_RATE);

        assert_eq!(actual, 0);
    }

    #[test]
    fn test_calculate_zero_fuel_cost_increasing_fuel_cost() {
        let input = vec![1];

        let actual = calculate_fuel_cost_for_move(1, &input, INCREASING_FUEL_RATE);

        assert_eq!(actual, 0);
    }

    #[test]
    fn test_calculate_fuel_cost_first_position() {
        let input = vec![1, 5, 6, 10];

        let actual = calculate_fuel_cost_for_move(1, &input, CONSTANT_FUEL_RATE);

        assert_eq!(actual, 18);
    }

    #[test]
    fn test_calculate_fuel_cost_middle_position() {
        let input = vec![1, 5, 6, 10];

        let actual = calculate_fuel_cost_for_move(6, &input, CONSTANT_FUEL_RATE);

        assert_eq!(actual, 10);
    }

    #[test]
    fn test_calculate_fuel_cost_first_position_increasing_fuel_cost() {
        let input = vec![1, 5, 6, 10];

        let actual = calculate_fuel_cost_for_move(1, &input, INCREASING_FUEL_RATE);

        let seventy =
            0 + (1 + 2 + 3 + 4) + (1 + 2 + 3 + 4 + 5) + (1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9);

        assert_eq!(actual, seventy);
    }

    #[test]
    fn test_calculate_fuel_cost_position_increasing_fuel_cost() {
        let input = vec![1, 5, 6, 10];

        let actual = calculate_fuel_cost_for_move(4, &input, INCREASING_FUEL_RATE);

        let thirty_one = (1 + 2 + 3) + (1) + (1 + 2) + (1 + 2 + 3 + 4 + 5 + 6);

        assert_eq!(actual, thirty_one);
    }
}
