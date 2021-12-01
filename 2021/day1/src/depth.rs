pub fn calculate_increased_depths(readings: Vec<u32>) -> u32 {
    readings
        .windows(2)
        .fold(0, |acc, x| if x[0] < x[1] { acc + 1 } else { acc })
}

#[cfg(test)]
mod tests {

    use crate::depth::*;

    #[test]
    fn calculate_zero_increases() {
        let readings: Vec<u32> = vec![100, 95];

        let actual_increases = calculate_increased_depths(readings);

        assert_eq!(actual_increases, 0);
    }

    #[test]
    fn calculate_increases() {
        let readings: Vec<u32> = vec![100, 101, 102, 101, 100];

        let actual_increases = calculate_increased_depths(readings);

        assert_eq!(actual_increases, 2);
    }
}
