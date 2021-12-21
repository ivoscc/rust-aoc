use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Eq, PartialEq, Hash, Clone, Debug, Copy)]
struct Position {
    risk: usize,
    row: usize,
    col: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.risk.cmp(&other.risk).reverse()
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(input: &str) -> Vec<Vec<Position>> {
    input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, character)| Position {
                    row,
                    col,
                    risk: character.to_digit(10).unwrap() as usize,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(373, output_part_1);
    assert_eq!(2868, output_part_2);
}

fn get_next_positions(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
    grid: &Vec<Vec<Position>>,
) -> Vec<Position> {
    let mut next_nodes = vec![];
    if row > 0 {
        next_nodes.push(grid[row - 1][col]);
    }
    if row < height - 1 {
        next_nodes.push(grid[row + 1][col]);
    }
    if col > 0 {
        next_nodes.push(grid[row][col - 1]);
    }
    if col < width - 1 {
        next_nodes.push(grid[row][col + 1]);
    }
    next_nodes
}

fn get_lowest_corner_to_corner_risk(risk_grid: &Vec<Vec<Position>>) -> usize {
    let height = risk_grid.len();
    let width = risk_grid[0].len();
    let initial = Position {
        risk: 0,
        ..risk_grid[0][0]
    };
    let target = risk_grid[height - 1][width - 1];

    let mut risks = HashMap::<Position, usize>::new();
    let mut heap = BinaryHeap::<Position>::new();
    risks.insert(initial, 0);
    heap.push(initial);

    while heap.len() > 0 {
        let Position { row, col, risk } = heap.pop().unwrap();
        for next_position in get_next_positions(row, col, width, height, &risk_grid) {
            let risk = risk + next_position.risk;
            let existing = risks.entry(next_position).or_insert(usize::MAX);
            if risk < *existing {
                *existing = risk;
                heap.push(Position {
                    risk,
                    ..next_position
                })
            }
        }
    }
    *risks.get(&target).unwrap()
}

fn part_1(input: &str) -> usize {
    let risk_grid = parse_input(input);
    get_lowest_corner_to_corner_risk(&risk_grid)
}

fn increase_grid_size(grid: &Vec<Vec<Position>>) -> Vec<Vec<Position>> {
    let width = grid[0].len();
    let height = grid.len();
    let mut output: Vec<Vec<Position>> = vec![];
    for row in 0..(5 * height) {
        let mut new_row = vec![];
        if row >= height {
            for col in 0..(5 * width) {
                let risk = (output[row - height][col].risk % 9) + 1;
                new_row.push(Position { row, col, risk })
            }
        } else {
            for col in 0..(5 * width) {
                if col >= width {
                    let v = grid[row][col % width].risk + (col / width);
                    let risk = (v - 1) % 9 + 1;
                    new_row.push(Position { row, col, risk });
                } else {
                    new_row.push(grid[row][col]);
                }
            }
        }
        output.push(new_row);
    }
    output
}

fn part_2(input: &str) -> usize {
    let risk_grid = parse_input(input);
    let risk_grid = increase_grid_size(&risk_grid);
    get_lowest_corner_to_corner_risk(&risk_grid)
}
