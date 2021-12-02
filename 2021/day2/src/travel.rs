pub fn determine_travel(directions: &Vec<(&str, u32)>) -> (u32, u32) {
    directions.iter().fold((0, 0), |acc, d| match d {
        ("forward", x) => (acc.0 + x, acc.1),
        ("down", x) => (acc.0, acc.1 + x),
        ("up", x) => (acc.0, acc.1 - x),
        (unknown, _) => {
            println!("Unknown command: {}, coords remain unchanged", unknown);

            acc
        }
    })
}

#[cfg(test)]
mod tests {

    use crate::travel::*;

    #[test]
    fn determine_zero_coords_for_no_travel() {
        let directions = vec![];

        let actual_coords = determine_travel(&directions);

        assert_eq!(actual_coords, (0, 0))
    }

    #[test]
    fn determine_coords_for_travel() {
        let directions = vec![
            ("forward", 5),
            ("down", 5),
            ("forward", 8),
            ("up", 3),
            ("down", 8),
            ("forward", 2),
        ];

        let actual_coords = determine_travel(&directions);

        assert_eq!(actual_coords, (15, 10))
    }
}
