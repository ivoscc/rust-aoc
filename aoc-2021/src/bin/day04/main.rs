use std::{collections::HashSet, fmt};

use itertools::Itertools;

struct Board(Vec<Vec<isize>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display = self
            .0
            .iter()
            .map(|line| line.iter().map(|n| format!("{: >2}", n)).join(" "))
            .join("\n");
        write!(f, "{}", display)
    }
}

impl Board {
    fn check_won(&self, row: usize, col: usize) -> bool {
        let board = &self.0;

        let mut complete_row = true;
        for index in 0..5 {
            if board[row][index] != -1 {
                complete_row = false;
                break;
            }
        }

        if complete_row {
            return true;
        }

        let mut complete_col = true;
        for index in 0..5 {
            if board[index][col] != -1 {
                complete_col = false;
                break;
            }
        }

        return complete_col;
    }

    fn mark_number(&mut self, target_number: isize) -> Option<(usize, usize)> {
        let board = &mut self.0;
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col] == target_number {
                    board[row][col] = -1;
                    return Some((row, col));
                }
            }
        }
        None
    }
}

fn parse_input(input: &str) -> (Vec<Board>, Vec<isize>) {
    let mut lines = input.lines();
    let numbers = lines
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            Board(
                chunk
                    .skip(1)
                    .map(|board_line| {
                        board_line
                            .split(" ")
                            .filter(|n| n.len() > 0)
                            .map(|n| n.parse::<isize>().unwrap())
                            .collect_vec()
                    })
                    .collect_vec(),
            )
        })
        .collect_vec();

    (boards, numbers)
}

fn get_score(board: &Board) -> usize {
    let Board(board) = board;
    board
        .iter()
        .map(|row| {
            row.iter()
                .filter(|n| **n != -1)
                .map(|n| *n as usize)
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(25410, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn part_1(input: &str) -> usize {
    let (mut boards, numbers) = parse_input(input);

    for number in numbers {
        for board in &mut boards {
            if let Some((row, col)) = board.mark_number(number) {
                if board.check_won(row, col) {
                    return get_score(&board) * number as usize;
                }
            }
        }
    }
    0
}

fn part_2(input: &str) -> usize {
    let (mut boards, numbers) = parse_input(input);
    let number_of_boards = boards.len();
    let mut won = HashSet::<usize>::new();

    for number in numbers {
        for (board_number, board) in (&mut boards).into_iter().enumerate() {
            if won.contains(&board_number) {
                continue;
            }
            if let Some((row, col)) = board.mark_number(number) {
                if board.check_won(row, col) {
                    won.insert(board_number);
                    if won.len() == number_of_boards {
                        return get_score(board) * number as usize;
                    }
                }
            }
        }
    }
    0
}
