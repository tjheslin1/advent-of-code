#[derive(Debug, PartialEq)]
pub struct BingoCard {
    grid: Vec<([usize; 2], u32, bool)>,
}

impl BingoCard {
    pub fn from_input(input: &[&str]) -> Self {
        let row_count = input.len();

        let grid = input
            .iter()
            .zip(0..row_count)
            .map(|(line, y)| {
                let mut row_numbers = line
                    .replace("  ", " ")
                    .trim()
                    .split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();

                let col_count = row_numbers.len();

                row_numbers
                    .iter_mut()
                    .zip(0..col_count)
                    .map(|(n, x)| ([x, y], *n, false))
                    // .into_iter()
                    .collect::<Vec<([usize; 2], u32, bool)>>()
            })
            .flatten()
            .collect::<Vec<([usize; 2], u32, bool)>>();

        BingoCard { grid }
    }
}

pub fn run_bingo(bingo_numbers: &str, mut cards: Vec<BingoCard>) -> &[u32] {
    // bingo_numbers
    //     .split(',')
    //     .map(|n| n.parse::<u32>().unwrap())
    //     .for_each(|bingo_num| {
    //         cards.iter_mut().for_each(|card| {
    //             let found_num = card.grid.iter_mut().find(|num| num == bingo_num);
    //         })
    //     });

    &[0; 1]
}

pub fn parse_bingo_input(input: &str) -> (&str, Vec<BingoCard>) {
    let lines: Vec<&str> = input.split('\n').collect();

    let (bingo_numbers, card_input) = match lines.split_first() {
        Some((first, rest)) => (first, rest),
        _ => (&"", &[] as &[&str]),
    };

    let cards = card_input
        .split(|line| line.is_empty())
        .filter(|line| line.is_empty() == false)
        .map(|card| BingoCard::from_input(card))
        .collect::<Vec<BingoCard>>();

    (bingo_numbers, cards)
}

#[cfg(test)]
mod tests {

    use crate::bingo::*;

    #[test]
    fn run_bingo_example() {
        let bingo_numbers =
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1";

        #[rustfmt::skip]
            let cards = vec![
            BingoCard { grid: vec![
                ([0,0], 22, false),([1,0], 13, false),([2,0], 17, false),([3,0], 11, false),([4,0], 0, false),
                ([0,1], 8, false),([1,1], 2, false),([2,1], 23, false),([3,1], 4, false),([4,1], 24, false),
                ([0,2], 21, false),([1,2], 9, false),([2,2], 14, false),([3,2], 16, false),([4,2], 7, false),
                ([0,3], 6, false),([1,3], 10, false),([2,3], 3, false),([3,3], 18, false),([4,3], 5, false),
                ([0,4], 1, false),([1,4], 12, false),([2,4], 20, false),([3,4], 15, false),([4,4], 19, false),
            ]},
            BingoCard { grid: vec![
                ([0,0], 3, false),([1,0], 15, false),([2,0], 0, false),([3,0], 2, false),([4,0], 22, false),
                ([0,1], 9, false),([1,1], 18, false),([2,1], 13, false),([3,1], 17, false),([4,1], 5, false),
                ([0,2], 19, false),([1,2], 8, false),([2,2], 7, false),([3,2], 25, false),([4,2], 23, false),
                ([0,3], 20, false),([1,3], 11, false),([2,3], 10, false),([3,3], 24, false),([4,3], 4, false),
                ([0,4], 14, false),([1,4], 21, false),([2,4], 16, false),([3,4], 12, false),([4,4], 6, false),
            ]},
            BingoCard { grid: vec![
                ([0,0], 14, false),([1,0], 21, false),([2,0], 17, false),([3,0], 24, false),([4,0], 4, false),
                ([0,1], 10, false),([1,1], 16, false),([2,1], 15, false),([3,1], 9, false),([4,1], 19, false),
                ([0,2], 18, false),([1,2], 8, false),([2,2], 23, false),([3,2], 26, false),([4,2], 20, false),
                ([0,3], 22, false),([1,3], 11, false),([2,3], 13, false),([3,3], 6, false),([4,3], 5, false),
                ([0,4], 2, false),([1,4], 0, false),([2,4], 12, false),([3,4], 3, false),([4,4], 7, false),
            ]},
            ];

        let actual_result = run_bingo(bingo_numbers, cards);

        let expected_result = vec![14, 21, 17, 24, 4];

        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn construct_empty_bingo_card_from_input() {
        let input = vec![];

        let actual_bingo_card = BingoCard::from_input(&input);

        let expected_bingo_card = BingoCard { grid: vec![] };

        assert_eq!(actual_bingo_card, expected_bingo_card);
    }

    #[test]
    fn construct_bingo_card_from_input() {
        let input = vec![
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ];

        let actual_bingo_card = BingoCard::from_input(&input);

        #[rustfmt::skip]
        let expected_bingo_card = BingoCard {
            grid: vec![
                ([0,0], 22, false),([1,0], 13, false),([2,0], 17, false),([3,0], 11, false),([4,0], 0, false),
                ([0,1], 8, false),([1,1], 2, false),([2,1], 23, false),([3,1], 4, false),([4,1], 24, false),
                ([0,2], 21, false),([1,2], 9, false),([2,2], 14, false),([3,2], 16, false),([4,2], 7, false),
                ([0,3], 6, false),([1,3], 10, false),([2,3], 3, false),([3,3], 18, false),([4,3], 5, false),
                ([0,4], 1, false),([1,4], 12, false),([2,4], 20, false),([3,4], 15, false),([4,4], 19, false),
            ],
        };

        assert_eq!(actual_bingo_card, expected_bingo_card);
    }

    #[test]
    fn parse_empty_bingo_input() {
        let input = "";

        let (bingo_numbers, cards) = parse_bingo_input(input);

        assert_eq!(bingo_numbers, "");
        assert_eq!(cards, vec![]);
    }

    #[test]
    fn parse_bingo_input_for_two_cards() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        #[rustfmt::skip]
        let expected_bingo_cards = vec![
        BingoCard { grid: vec![
            ([0,0], 22, false),([1,0], 13, false),([2,0], 17, false),([3,0], 11, false),([4,0], 0, false),
            ([0,1], 8, false),([1,1], 2, false),([2,1], 23, false),([3,1], 4, false),([4,1], 24, false),
            ([0,2], 21, false),([1,2], 9, false),([2,2], 14, false),([3,2], 16, false),([4,2], 7, false),
            ([0,3], 6, false),([1,3], 10, false),([2,3], 3, false),([3,3], 18, false),([4,3], 5, false),
            ([0,4], 1, false),([1,4], 12, false),([2,4], 20, false),([3,4], 15, false),([4,4], 19, false),
        ]},
        BingoCard { grid: vec![
            ([0,0], 3, false),([1,0], 15, false),([2,0], 0, false),([3,0], 2, false),([4,0], 22, false),
            ([0,1], 9, false),([1,1], 18, false),([2,1], 13, false),([3,1], 17, false),([4,1], 5, false),
            ([0,2], 19, false),([1,2], 8, false),([2,2], 7, false),([3,2], 25, false),([4,2], 23, false),
            ([0,3], 20, false),([1,3], 11, false),([2,3], 10, false),([3,3], 24, false),([4,3], 4, false),
            ([0,4], 14, false),([1,4], 21, false),([2,4], 16, false),([3,4], 12, false),([4,4], 6, false),
        ]},
        BingoCard { grid: vec![
            ([0,0], 14, false),([1,0], 21, false),([2,0], 17, false),([3,0], 24, false),([4,0], 4, false),
            ([0,1], 10, false),([1,1], 16, false),([2,1], 15, false),([3,1], 9, false),([4,1], 19, false),
            ([0,2], 18, false),([1,2], 8, false),([2,2], 23, false),([3,2], 26, false),([4,2], 20, false),
            ([0,3], 22, false),([1,3], 11, false),([2,3], 13, false),([3,3], 6, false),([4,3], 5, false),
            ([0,4], 2, false),([1,4], 0, false),([2,4], 12, false),([3,4], 3, false),([4,4], 7, false),
        ]},
        ];

        let (bingo_numbers, cards) = parse_bingo_input(input);

        assert_eq!(
            bingo_numbers,
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1"
        );
        assert_eq!(cards, expected_bingo_cards);
    }
}
