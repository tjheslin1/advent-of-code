use itertools::Itertools;

type Point = (usize, usize);
type Line = (usize, usize, usize, usize);

pub fn find_overlapping_points(lines: &[Line], min_overlap: usize, handle_diagonal: bool) -> usize {
    let mut overlaps: Vec<Point> = vec![];

    for first in 0..lines.len() {
        for second in first + 1..lines.len() {
            overlaps.append(&mut lines_overlap(
                &lines[first],
                &lines[second],
                handle_diagonal,
            ));
        }
    }

    overlaps.into_iter().unique().count()
}

pub fn parse_input(input: &str) -> Vec<Line> {
    input
        .split('\n')
        .filter(|line| line.is_empty() == false)
        .map(|line| {
            let x = line
                .split(" -> ")
                .flat_map(|pair| {
                    pair.split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<usize>>();

            match &x[..] {
                [a, b, c, d] => (*a, *b, *c, *d),
                x => panic!("Unexpected list of nums found: {:?}", x),
            }
        })
        .collect::<Vec<Line>>()
}

fn lines_overlap(first: &Line, second: &Line, handle_diagonal: bool) -> Vec<Point> {
    let (first_x_1, first_y_1, first_x_2, first_y_2) = *first;
    let (second_x_1, second_y_1, second_x_2, second_y_2) = *second;

    if handle_diagonal == false && first_x_1 != first_x_2 && first_y_1 != first_y_2 {
        return vec![];
    }

    if handle_diagonal == false && second_x_1 != second_x_2 && second_y_1 != second_y_2 {
        return vec![];
    }

    let first_points = find_points(first_x_1, first_y_1, first_x_2, first_y_2);
    let second_points = find_points(second_x_1, second_y_1, second_x_2, second_y_2);

    first_points
        .iter()
        .filter(|p| second_points.contains(*p))
        .copied()
        .collect::<Vec<Point>>()
}

fn find_points(x_1: usize, y_1: usize, x_2: usize, y_2: usize) -> Vec<Point> {
    let x_length = i32::abs((x_1 as i32) - (x_2 as i32)) as usize;
    let y_length = i32::abs((y_1 as i32) - (y_2 as i32)) as usize;

    let x_range = match x_length {
        0 => vec![x_1; y_length + 1],
        _ if x_1 <= x_2 => (x_1..=x_2).collect::<Vec<usize>>(),
        _ => (x_2..=x_1).rev().collect::<Vec<usize>>(),
    };

    let y_range = match y_length {
        0 => vec![y_1; x_length + 1],
        _ if y_1 <= y_2 => (y_1..=y_2).collect::<Vec<usize>>(),
        _ => (y_2..=y_1).rev().collect::<Vec<usize>>(),
    };

    x_range
        .iter()
        .zip(y_range.iter())
        .map(|(x, y)| (*x, *y))
        .collect::<Vec<Point>>()
}

#[cfg(test)]
mod tests {

    use crate::vents::*;

    #[test]
    fn test_parse_input_example() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

        let parsed_actual = parse_input(input);
        let expected_actual = vec![
            (0, 9, 5, 9),
            (8, 0, 0, 8),
            (9, 4, 3, 4),
            (2, 2, 2, 1),
            (7, 0, 7, 4),
            (6, 4, 2, 0),
            (0, 9, 2, 9),
            (3, 4, 1, 4),
            (0, 0, 8, 8),
            (5, 5, 8, 2),
        ];

        assert_eq!(parsed_actual, expected_actual);
    }

    #[test]
    fn test_find_overlapping_points_example() {
        let input_points = vec![
            (0, 9, 5, 9),
            (8, 0, 0, 8),
            (9, 4, 3, 4),
            (2, 2, 2, 1),
            (7, 0, 7, 4),
            (6, 4, 2, 0),
            (0, 9, 2, 9),
            (3, 4, 1, 4),
            (0, 0, 8, 8),
            (5, 5, 8, 2),
        ];

        let actual_num_overlapping_points = find_overlapping_points(&input_points[..], 2, false);

        assert_eq!(actual_num_overlapping_points, 5);
    }

    #[test]
    fn test_find_overlapping_points_diagonal_example() {
        let input_points = vec![
            (0, 9, 5, 9),
            (8, 0, 0, 8),
            (9, 4, 3, 4),
            (2, 2, 2, 1),
            (7, 0, 7, 4),
            (6, 4, 2, 0),
            (0, 9, 2, 9),
            (3, 4, 1, 4),
            (0, 0, 8, 8),
            (5, 5, 8, 2),
        ];

        let actual_num_overlapping_points = find_overlapping_points(&input_points[..], 2, true);

        assert_eq!(actual_num_overlapping_points, 12);
    }

    /*
    1.........
    1.........
    1.........
    ..........
    */
    #[test]
    fn test_find_points_vertical() {
        let actual = find_points(0, 0, 0, 2);
        let expected = vec![(0, 0), (0, 1), (0, 2)];

        assert_eq!(actual, expected);
    }

    /*
    111.......
    ..........
    ..........
    ..........
    */
    #[test]
    fn test_find_points_horizontal() {
        let actual = find_points(0, 0, 2, 0);
        let expected = vec![(0, 0), (1, 0), (2, 0)];

        assert_eq!(actual, expected);
    }

    /*
    1.........
    .1........
    ..1.......
    ..........
    */
    #[test]
    fn test_find_points_diagonal() {
        let actual = find_points(0, 0, 2, 2);
        let expected = vec![(0, 0), (1, 1), (2, 2)];

        assert_eq!(actual, expected);
    }

    /*
    ..1.......
    .1........
    1.........
    ..........
    */
    #[test]
    fn test_find_points_diagonal_back() {
        let actual = find_points(2, 0, 0, 2);
        let expected = vec![(2, 0), (1, 1), (0, 2)];

        assert_eq!(actual, expected);
    }

    /*
    1.1.......
    1.1.......
    1.1.......
    ..........
    */
    #[test]
    fn test_do_lines_overlap_false() {
        let first = (0, 0, 0, 2);
        let second = (2, 0, 2, 2);

        let actual = lines_overlap(&first, &second, false);
        let expected = vec![];

        assert_eq!(actual, expected);
    }

    /*
    1.........
    2111......
    1.........
    ..........
    */
    #[test]
    fn test_do_lines_overlap_vertical_cross_horizontal() {
        let first = (0, 0, 0, 2);
        let second = (0, 1, 3, 1);

        let actual = lines_overlap(&first, &second, false);
        let expected = vec![(0, 1)];

        assert_eq!(actual, expected);
    }

    /*
    112.......
    ..1.......
    ..1.......
    ..........
    */
    #[test]
    fn test_do_lines_overlap_horizontal_cross_vertical() {
        let first = (0, 0, 2, 0);
        let second = (2, 0, 2, 2);

        let actual = lines_overlap(&first, &second, false);
        let expected = vec![(2, 0)];

        assert_eq!(actual, expected);
    }

    /*
    ..2.......
    ..2.......
    ..1.......
    ..........
    */
    #[test]
    fn test_do_lines_overlap_line_on_top_of_line() {
        let first = (2, 0, 2, 2);
        let second = (2, 0, 2, 1);

        let actual = lines_overlap(&first, &second, false);
        let expected = vec![(2, 0), ((2, 1))];

        assert_eq!(actual, expected);
    }

    /*
    1.1.......
    .2........
    1.1.......
    ..........
    */
    #[test]
    fn test_do_lines_overlap_diagonals() {
        let first = (2, 0, 0, 2);
        let second = (0, 0, 2, 2);

        let actual = lines_overlap(&first, &second, true);
        let expected = vec![(1, 1)];

        assert_eq!(actual, expected);
    }
}
