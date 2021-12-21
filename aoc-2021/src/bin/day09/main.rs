use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    // assert_eq!(_, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn parse_heigh_map(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|n: char| n.to_digit(10).unwrap() as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn part_1(input: &str) -> usize {
    let height_map = parse_heigh_map(input);
    let height = height_map.len();
    let width = height_map[0].len();

    let mut risk_levels = 0;
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, location) in row.iter().enumerate() {
            if row_index >= 1 && height_map[row_index - 1][col_index] <= *location {
                continue;
            }
            if row_index + 1 < height && height_map[row_index + 1][col_index] <= *location {
                continue;
            }
            if col_index >= 1 && height_map[row_index][col_index - 1] <= *location {
                continue;
            }
            if col_index + 1 < width && height_map[row_index][col_index + 1] <= *location {
                continue;
            }
            risk_levels += 1 + *location;
        }
    }
    risk_levels
}

fn get_basin_size(
    height_map: &Vec<Vec<usize>>,
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    queue.push_front((row, col));
    while queue.len() > 0 {
        let (row, col) = queue.pop_back().unwrap();
        seen.insert((row, col));
        if row > 0 {
            let pos = (row - 1, col);
            if !seen.contains(&pos) && height_map[row - 1][col] != 9 {
                queue.push_front(pos);
            }
        }
        if col > 0 {
            let pos = (row, col - 1);
            if !seen.contains(&pos) && height_map[row][col - 1] != 9 {
                queue.push_front(pos);
            }
        }
        if row + 1 < height {
            let pos = (row + 1, col);
            if !seen.contains(&pos) && height_map[row + 1][col] != 9 {
                queue.push_front(pos);
            }
        }
        if col + 1 < width {
            let pos = (row, col + 1);
            if !seen.contains(&pos) && height_map[row][col + 1] != 9 {
                queue.push_front(pos);
            }
        }
    }
    seen.len()
}

fn part_2(input: &str) -> usize {
    let height_map = parse_heigh_map(input);
    let height = height_map.len();
    let width = height_map[0].len();

    let mut basin_sizes = vec![];
    for (row_index, row) in height_map.iter().enumerate() {
        for (col_index, location) in row.iter().enumerate() {
            if row_index >= 1 && height_map[row_index - 1][col_index] <= *location {
                continue;
            }
            if row_index + 1 < height && height_map[row_index + 1][col_index] <= *location {
                continue;
            }
            if col_index >= 1 && height_map[row_index][col_index - 1] <= *location {
                continue;
            }
            if col_index + 1 < width && height_map[row_index][col_index + 1] <= *location {
                continue;
            }
            let basin_size = get_basin_size(&height_map, row_index, col_index, width, height);
            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}
