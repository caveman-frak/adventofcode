use {
    anyhow::Result,
    common::{
        convert::{to_u32, to_vec},
        input::{batch, from_path, list},
    },
    std::collections::HashMap,
};

fn main() -> Result<()> {
    let inputs = list(to_u32, from_path("day4/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[u32]) -> Result<()> {
    let boards = &mut boards_from_path("day4/data/boards.txt")?;

    if let Some((number, board)) = play(inputs, boards) {
        println!(
            "Day 4 Part 1 => {}",
            number * board.unmarked().iter().copied().sum::<u32>()
        );
    } else {
        println!("Day 4 Part 1 => No winning board!");
    }
    Ok(())
}

fn part2(inputs: &[u32]) -> Result<()> {
    let boards = &mut boards_from_path("day4/data/boards.txt")?;

    if let Some((number, board)) = play_last(inputs, boards) {
        println!(
            "Day 4 Part 2 => {}",
            number * board.unmarked().iter().copied().sum::<u32>()
        );
    } else {
        println!("Day 4 Part 2 => No winning board!");
    }
    Ok(())
}

fn to_vec_u32(s: String) -> Option<Vec<u32>> {
    if s.is_empty() {
        None
    } else {
        Some(to_vec(s, ' ', to_u32))
    }
}

fn boards_from_path(path: &str) -> Result<Vec<Board>> {
    Ok(batch(to_vec_u32, |v| v.is_none(), from_path(path)?)
        .iter()
        .map(|v| Board::new(v))
        .collect())
}

#[derive(Debug, Clone, PartialEq)]
struct Number {
    row: usize,
    column: usize,
    marked: bool,
}

impl Number {
    fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Board {
    numbers: HashMap<u32, Number>,
    rows: Vec<usize>,
    columns: Vec<usize>,
    won: bool,
}

impl Board {
    fn new(values: &[Vec<u32>]) -> Self {
        let mut board = Self {
            numbers: HashMap::new(),
            rows: vec![0; 5],
            columns: vec![0; 5],
            won: false,
        };
        board.fill(values);
        board
    }

    fn fill(&mut self, values: &[Vec<u32>]) {
        values
            .iter()
            .enumerate()
            .for_each(|(row, values)| self.fill_row(row, values));
    }

    fn fill_row(&mut self, row: usize, values: &[u32]) {
        values
            .iter()
            .enumerate()
            .for_each(|(column, value)| self.fill_cell(*value, row, column));
    }

    fn fill_cell(&mut self, value: u32, row: usize, column: usize) {
        self.numbers.insert(value, Number::new(row, column));
    }

    fn draw(&mut self, number: &u32) -> Option<bool> {
        self.numbers.get_mut(number).map(|number| {
            if !self.won {
                number.mark();
                let Number { row, column, .. } = number;
                let r = self.rows[*row] + 1;
                self.rows[*row] = r;
                let c = self.columns[*column] + 1;
                self.columns[*column] = c;
                if r == 5 || c == 5 {
                    self.won = true
                }
                self.won
            } else {
                false
            }
        })
    }

    fn unmarked(&self) -> Vec<u32> {
        self.numbers
            .iter()
            .filter(|(_, v)| !v.marked)
            .map(|(k, _)| *k)
            .collect()
    }
}

fn play(numbers: &[u32], boards: &mut [Board]) -> Option<(u32, Board)> {
    numbers
        .iter()
        .filter_map(|n| {
            boards
                .iter_mut()
                .filter_map(|board| match board.draw(n) {
                    Some(true) => Some((*n, board.clone())),
                    _ => None,
                })
                .next()
        })
        .next()
}

fn play_last(numbers: &[u32], boards: &mut [Board]) -> Option<(u32, Board)> {
    numbers
        .iter()
        .filter_map(|n| {
            boards
                .iter_mut()
                .filter_map(|board| match board.draw(n) {
                    Some(true) => Some((*n, board.clone())),
                    _ => None,
                })
                .last()
        })
        .last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fill() {
        let board = Board::new(&vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ]);
        assert_eq!(board.rows[4], 0);
        assert_eq!(board.columns[4], 0);
        assert_eq!(board.numbers.get(&30), None);
        assert_eq!(board.numbers.get(&10), Some(&Number::new(3, 1)));
    }

    #[test]
    fn check_draw() {
        let mut board = Board::new(&vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ]);
        assert_eq!(board.draw(&30), None);
        assert_eq!(board.draw(&13), Some(false));
        assert_eq!(board.numbers.get(&13).unwrap().marked, true);
        assert_eq!(board.draw(&2), Some(false));
        assert_eq!(board.draw(&9), Some(false));
        assert_eq!(board.draw(&10), Some(false));
        assert_eq!(board.draw(&12), Some(true));
        assert_eq!(board.columns[1], 5);
    }

    #[test]
    fn check_winning_board() {
        let mut board = Board::new(&vec![
            vec![14, 21, 17, 24, 4],
            vec![10, 16, 15, 9, 19],
            vec![18, 8, 23, 26, 20],
            vec![22, 11, 13, 6, 5],
            vec![2, 0, 12, 3, 7],
        ]);
        let draws = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        assert_eq!(
            draws
                .iter()
                .map(|n| (*n, board.draw(n)))
                .collect::<Vec<(u32, Option<bool>)>>(),
            vec![
                (7, Some(false)),
                (4, Some(false)),
                (9, Some(false)),
                (5, Some(false)),
                (11, Some(false)),
                (17, Some(false)),
                (23, Some(false)),
                (2, Some(false)),
                (0, Some(false)),
                (14, Some(false)),
                (21, Some(false)),
                (24, Some(true)),
            ]
        );
        assert_eq!(board.unmarked().iter().map(|n| *n).sum::<u32>(), 188);
    }

    #[test]
    fn check_loosing_board() {
        let mut board = Board::new(&vec![
            vec![22, 13, 17, 11, 0],
            vec![8, 2, 23, 4, 24],
            vec![21, 9, 14, 16, 7],
            vec![6, 10, 3, 18, 5],
            vec![1, 12, 20, 15, 19],
        ]);
        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        assert_eq!(
            numbers
                .iter()
                .map(|n| (*n, board.draw(n)))
                .collect::<Vec<(u32, Option<bool>)>>(),
            vec![
                (7, Some(false)),
                (4, Some(false)),
                (9, Some(false)),
                (5, Some(false)),
                (11, Some(false)),
                (17, Some(false)),
                (23, Some(false)),
                (2, Some(false)),
                (0, Some(false)),
                (14, Some(false)),
                (21, Some(false)),
                (24, Some(false)),
            ]
        );
        assert_eq!(board.unmarked().iter().map(|n| *n).sum::<u32>(), 163);
    }

    #[test]
    fn check_play() {
        let boards = &mut vec![
            Board::new(&vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(&vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(&vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let numbers = &vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let result = play(numbers, boards);

        assert!(matches!(result, Some((24, _))));
        assert_eq!(
            result.unwrap().1.unmarked().iter().map(|n| *n).sum::<u32>(),
            188
        );
    }

    #[test]
    fn check_play_fail() {
        let boards = &mut vec![
            Board::new(&vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(&vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(&vec![
                vec![14, 21, 17, 29, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let numbers = &vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];
        let result = play(numbers, boards);

        assert!(matches!(result, None));
    }

    #[test]
    fn check_play_last() {
        let boards = &mut vec![
            Board::new(&vec![
                vec![22, 13, 17, 11, 0],
                vec![8, 2, 23, 4, 24],
                vec![21, 9, 14, 16, 7],
                vec![6, 10, 3, 18, 5],
                vec![1, 12, 20, 15, 19],
            ]),
            Board::new(&vec![
                vec![3, 15, 0, 2, 22],
                vec![9, 18, 13, 17, 5],
                vec![19, 8, 7, 25, 23],
                vec![20, 11, 10, 24, 4],
                vec![14, 21, 16, 12, 6],
            ]),
            Board::new(&vec![
                vec![14, 21, 17, 24, 4],
                vec![10, 16, 15, 9, 19],
                vec![18, 8, 23, 26, 20],
                vec![22, 11, 13, 6, 5],
                vec![2, 0, 12, 3, 7],
            ]),
        ];
        let numbers = &vec![
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ];
        let result = play_last(numbers, boards);
        println!("{:?}", result);

        assert!(matches!(result, Some((13, _))));
        assert_eq!(
            result.unwrap().1.unmarked().iter().map(|n| *n).sum::<u32>(),
            148
        );
    }

    #[test]
    fn check_boards_from_path() -> Result<()> {
        let boards = boards_from_path("test/boards.txt")?;
        assert_eq!(boards.len(), 3);
        Ok(())
    }
}
