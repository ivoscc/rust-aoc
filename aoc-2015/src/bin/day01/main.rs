use std::ops::ControlFlow;

fn part_1(input: &str) -> i32 {
    input
        .chars()
        .fold(0, |current_floor, next_move| {
            current_floor + match next_move {
                '(' => 1,
                ')' => -1,
                _ => 0
            }
        })
}

fn part_2(input: &str) -> i32 {
    let result = input
        .chars()
        .enumerate()
        .try_fold(0, |current_floor, (index, next_move)| {
            let next_floor = current_floor + match next_move {
                '(' => 1,
                ')' => -1,
                _ => 0
            };
            match next_floor {
                -1 => ControlFlow::Break((index + 1) as i32),
                _ => ControlFlow::Continue(next_floor)
            }
        });
    match result {
        ControlFlow::Break(index) => index,
        _ => 0
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    let output_part_2 = part_2(input);
    println!("Part 1 output is {:?}", output_part_1);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(280, output_part_1);
    assert_eq!(1797, output_part_2);
}
