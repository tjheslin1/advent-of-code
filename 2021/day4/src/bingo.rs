#[derive(Debug, PartialEq)]
pub struct BingoCard {
    grid: Vec<Vec<(u32, bool)>>,
}

impl BingoCard {
    pub fn from_input(input: &[&str]) -> Self {
        let grid = input
            .iter()
            .map(|line| {
                line.replace("  ", " ")
                    .trim()
                    .split(' ')
                    .map(|n| n.parse::<u32>().unwrap())
                    .map(|n| (n, false))
                    .collect::<Vec<(u32, bool)>>()
            })
            .collect::<Vec<Vec<(u32, bool)>>>();

        BingoCard { grid }
    }
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

    // ("", vec![])
    (bingo_numbers, cards)
}

#[cfg(test)]
mod tests {

    use crate::bingo::*;

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
                vec![(22, false),(13, false),(17, false),(11, false),(0, false),],
                vec![(8, false),(2, false),(23, false),(4, false),(24, false),],
                vec![(21, false),(9, false),(14, false),(16, false),(7, false),],
                vec![(6, false),(10, false),(3, false),(18, false),(5, false),],
                vec![(1, false),(12, false),(20, false),(15, false),(19, false),],
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
14 21 16 12  6";

        #[rustfmt::skip]
        let expected_bingo_cards = vec![
        BingoCard { grid: vec![
            vec![(22, false),(13, false),(17, false),(11, false),(0, false),],
            vec![(8, false),(2, false),(23, false),(4, false),(24, false),],
            vec![(21, false),(9, false),(14, false),(16, false),(7, false),],
            vec![(6, false),(10, false),(3, false),(18, false),(5, false),],
            vec![(1, false),(12, false),(20, false),(15, false),(19, false),],
        ]},
        BingoCard { grid: vec![
            vec![(3, false),(15, false),(0, false),(2, false),(22, false),],
            vec![(9, false),(18, false),(13, false),(17, false),(5, false),],
            vec![(19, false),(8, false),(7, false),(25, false),(23, false),],
            vec![(20, false),(11, false),(10, false),(24, false),(4, false),],
            vec![(14, false),(21, false),(16, false),(12, false),(6, false),],
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
