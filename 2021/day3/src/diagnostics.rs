pub fn diagnostics_report(readings: &[&str]) -> (isize, isize) {
    let reading_length = if readings.len() == 0 {
        0
    } else {
        readings.get(0).unwrap().len()
    };

    let column_bits: Vec<String> =
        (0..reading_length).fold(vec![String::new(); reading_length], |mut acc, index| {
            readings.iter().for_each(|line| {
                acc.get_mut(index)
                    .unwrap()
                    .push(line.chars().nth(index).unwrap());
            });

            acc
        });

    let most_common = column_bits
        .iter()
        .fold((String::from("0"), String::from("0")), |acc, column| {
            let count_zeroes = column.chars().filter(|char| *char == '0').count();

            if count_zeroes >= (column.len() / 2) {
                (acc.0 + "0", acc.1 + "1")
            } else {
                (acc.0 + "1", acc.1 + "0")
            }
        });

    match most_common {
        (b_gamma, b_epsilon) => {
            let gamma = isize::from_str_radix(&b_gamma, 2)
                .expect(&format!("Expected a binary input but got: {}", b_gamma)[..]);

            let epsilon = isize::from_str_radix(&b_epsilon, 2)
                .expect(&format!("Expected a binary input but got: {}", b_epsilon)[..]);

            (gamma, epsilon)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::diagnostics::*;

    #[test]
    fn empty_diagnostics_report() {
        let readings = vec![];

        let actual_report = diagnostics_report(&readings);

        assert_eq!(actual_report, (0, 0))
    }

    #[test]
    fn sample_diagnostics_report() {
        let readings = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let actual_report = diagnostics_report(&readings);

        assert_eq!(actual_report, (22, 9))
    }
}
