pub fn calculate_crabmarine_horizontal_position(input: &str) -> usize {
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

    0
}

#[cfg(test)]
mod tests {

    use crate::crabmarine::*;

    #[test]
    fn test_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";

        let actual = calculate_crabmarine_horizontal_position(input);

        assert_eq!(actual, 2)
    }
}
