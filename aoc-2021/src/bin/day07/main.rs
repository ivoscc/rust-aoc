use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(356922, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn parse_positions(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .sorted()
        .collect::<Vec<usize>>()
}

fn part_1(input: &str) -> usize {
    let positions = parse_positions(input);
    let median = if positions.len() % 2 == 0 {
        positions[(positions.len() / 2) - 1]
    } else {
        positions[positions.len() / 2]
    };
    positions
        .iter()
        .map(|v| (*v as isize - median as isize).abs() as usize)
        .sum()
}

fn part_2(input: &str) -> usize {
    let positions = parse_positions(input);
    let mut min_fuel_usage = 0;
    for alignmnent in 0..=positions[positions.len() - 1] {
        let mut fuel = 0;
        for position in &positions {
            let distance = (*position as isize - alignmnent as isize).abs() as usize;
            fuel += (distance * (distance + 1)) / 2
        }
        if min_fuel_usage == 0 || fuel < min_fuel_usage {
            min_fuel_usage = fuel;
        }
    }
    min_fuel_usage
}
