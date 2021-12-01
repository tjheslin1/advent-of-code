pub fn calculate_increased_depths(readings: Vec<u32>) -> u32 {
    0
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
}
