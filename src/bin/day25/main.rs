use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(8997277, output_part_1);
    // assert_eq!(_, output_part_2);
}

#[derive(PartialEq)]
struct Coordinates {
    row: usize,
    col: usize,
}

fn get_next_coordinates(current: &Coordinates) -> Coordinates {
    match (current.row, current.col) {
        (1, _) => Coordinates {
            col: 1,
            row: current.col + 1,
        },
        _ => Coordinates {
            col: current.col + 1,
            row: current.row - 1,
        },
    }
}

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut parsed = vec![];
    for line in input.lines().skip(2) {
        parsed.push(
            line.split(" ")
                .skip(3)
                .filter(|item| !item.is_empty())
                .map(|num| num.parse().unwrap())
                .collect_vec(),
        );
    }
    parsed
}

fn next_number(number: usize) -> usize {
    (number * 252533) % 33554393
}

fn part_1(input: &str) -> usize {
    let codes = parse_input(input);
    let mut coordinates = Coordinates { row: 1, col: 1 };
    let mut current_number = 20151125;
    let target_coordinates = Coordinates {
        row: 3010,
        col: 3019,
    };
    while coordinates != target_coordinates {
        current_number = next_number(current_number);
        coordinates = get_next_coordinates(&coordinates);
    }
    current_number
}

fn part_2(input: &str) -> () {}
