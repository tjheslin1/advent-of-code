pub fn calculate_increased_depths(readings: &[u32]) -> u32 {
    calculate_increased_depths_with_window(1, readings)
}

pub fn calculate_three_measurement_increased_depths(readings: &[u32]) -> u32 {
    calculate_increased_depths_with_window(3, readings)
}

fn calculate_increased_depths_with_window(window_size: usize, readings: &[u32]) -> u32 {
    let measurement_windows: Vec<&[u32]> = readings.windows(window_size).collect();

    measurement_windows.windows(2).fold(0, |acc, w| {
        if w[0].iter().sum::<u32>() < w[1].iter().sum::<u32>() {
            acc + 1
        } else {
            acc
        }
    })
}

#[cfg(test)]
mod tests {

    use crate::depth::*;

    #[test]
    fn calculate_zero_increases() {
        let readings: Vec<u32> = vec![100, 95];

        let actual_increases = calculate_increased_depths(&readings);

        assert_eq!(actual_increases, 0);
    }

    #[test]
    fn calculate_increases() {
        let readings: Vec<u32> = vec![100, 101, 102, 101, 100];

        let actual_increases = calculate_increased_depths(&readings);

        assert_eq!(actual_increases, 2);
    }

    #[test]
    fn caluclate_three_measurement_zero_increases() {
        let readings: Vec<u32> = vec![600, 599, 598, 597, 596, 595];

        let actual_increases = calculate_three_measurement_increased_depths(&readings);

        assert_eq!(actual_increases, 0);
    }

    #[test]
    fn caluclate_three_measurement_increases() {
        let readings: Vec<u32> = vec![607, 618, 618, 617, 647, 716, 769, 792];

        let actual_increases = calculate_three_measurement_increased_depths(&readings);

        assert_eq!(actual_increases, 5);
    }
}
