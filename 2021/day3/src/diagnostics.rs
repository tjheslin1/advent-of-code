pub fn diagnostics_report(readings: &[&str]) -> (isize, isize) {
    let initialised_readings = (String::from("0"), String::from("0"));

    let most_common = column_bits(readings)
        .iter()
        .fold(initialised_readings, |acc, column| {
            if most_common_bit(column, '0') == '0' {
                (acc.0 + "0", acc.1 + "1")
            } else {
                (acc.0 + "1", acc.1 + "0")
            }
        });

    match most_common {
        (b_gamma, b_epsilon) => {
            let gamma = binary_to_decimal(&b_gamma);

            let epsilon = binary_to_decimal(&b_epsilon);

            (gamma, epsilon)
        }
    }
}

pub fn life_support_rating(readings: &[&str]) -> (isize, isize) {
    let oxygen_rating = find_by_bit_criteria(readings, true, '1');
    let co2_rating = find_by_bit_criteria(readings, false, '0');

    (oxygen_rating, co2_rating)
}

fn binary_to_decimal(binary: &str) -> isize {
    isize::from_str_radix(binary, 2)
        .expect(&format!("Expected a binary input but got: {}", binary)[..])
}

fn find_by_bit_criteria(readings: &[&str], most_common: bool, on_match: char) -> isize {
    let readings_length = readings_length(readings);

    let reduced_readings = (0..readings_length).fold(readings.to_owned(), |acc, index| {
        let bit_to_match = find_common_bit(
            most_common,
            column_bits(&acc[..]).get(index).unwrap(),
            on_match,
        );

        if acc.len() > 1 {
            acc.iter()
                .filter(|line| line.chars().nth(index).unwrap() == bit_to_match)
                .copied()
                .collect::<Vec<&str>>()
        } else {
            acc
        }
    });

    match reduced_readings[..] {
        [binary] => binary_to_decimal(binary),
        [_, _] => panic!("Unable to reduce down to a single binary value!"),
        _ => 0,
    }
}

fn readings_length(readings: &[&str]) -> usize {
    if readings.is_empty() {
        0
    } else {
        readings.get(0).unwrap().len()
    }
}

fn most_common_bit(column: &str, on_match: char) -> char {
    find_common_bit(true, column, on_match)
}

fn find_common_bit(most_common: bool, column: &str, on_match: char) -> char {
    let count_zeroes = column.chars().filter(|char| *char == '0').count();

    if count_zeroes == (column.len() / 2) {
        on_match
    } else if count_zeroes > (column.len() / 2) {
        if most_common {
            '0'
        } else {
            '1'
        }
    } else if most_common {
        '1'
    } else {
        '0'
    }
}

fn column_bits(readings: &[&str]) -> Vec<String> {
    let readings_length = readings_length(readings);

    (0..readings_length).fold(vec![String::new(); readings_length], |mut acc, index| {
        readings.iter().for_each(|line| {
            acc.get_mut(index)
                .unwrap()
                .push(line.chars().nth(index).unwrap());
        });

        acc
    })
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

    #[test]
    fn empty_life_support_rating() {
        let readings = vec![];

        let actual_ratings = life_support_rating(&readings);

        assert_eq!(actual_ratings, (0, 0))
    }

    #[test]
    fn sample_life_support_rating() {
        let readings = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let actual_ratings = life_support_rating(&readings);

        assert_eq!(actual_ratings, (23, 10))
    }
}
