use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

fn update_position(current_position: Position, direction: char) -> Position {
    match direction {
        '>' => Position { x: current_position.x + 1, y: current_position.y },
        '<' => Position { x: current_position.x - 1, y: current_position.y },
        '^' => Position { x: current_position.x, y: current_position.y + 1},
        'v' => Position { x: current_position.x, y: current_position.y -1 },
        _ => current_position
    }
}

fn part_1(input: &str) -> usize {
    let initial_position = Position { x: 0, y: 0 };
    let (_, mut visited) = input.chars().fold(
        (initial_position, HashSet::new()),
        |(current_position, mut visited), direction| {
            let new_position = update_position(current_position, direction);
            visited.insert(new_position);
            (new_position, visited)
        },
    );
    visited.insert(initial_position);
    visited.len()
}

fn part_2(input: &str) -> usize {
    let santa_initial_position = Position { x: 0, y: 0 };
    let robot_initial_position = Position { x: 0, y: 0 };
    let (_, mut visited) = input.chars().enumerate().fold(
        ((santa_initial_position, robot_initial_position), HashSet::new()),
        |((santa_position, robot_position), mut visited), (index, direction)| {
            let (new_santa_position, new_robot_position) = if index % 2 == 0 {
                (update_position(santa_position, direction), robot_position)
            } else {
                (santa_position, update_position(robot_position, direction))
            };
            visited.insert(new_santa_position);
            visited.insert(new_robot_position);
            ((new_santa_position, new_robot_position), visited)
        },
    );
    visited.insert(santa_initial_position);
    visited.len()
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    let output_part_2 = part_2(input);
    println!("Part 1 output is {:?}", output_part_1);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(2572, output_part_1);
    assert_eq!(2631, output_part_2);
}
