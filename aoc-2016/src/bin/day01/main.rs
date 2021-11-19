use std::{
    cmp,
    collections::HashSet,
    fmt::{self, Display},
};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(271, output_part_1);
    assert_eq!(153, output_part_2);
}

#[derive(Debug)]
enum RelativeDirection {
    Left(i64),
    Right(i64),
}

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

struct Context {
    facing: Direction,
    coordinates: Coordinates,
}

fn parse_input(input: &str) -> Vec<RelativeDirection> {
    input
        .trim()
        .split(", ")
        .map(|Relativedirection| {
            let magnitude = &Relativedirection[1..]
                .parse::<i64>()
                .expect("Unable to parse direction magnitude.");
            match Relativedirection
                .chars()
                .next()
                .expect("Found empty direction")
            {
                'R' => RelativeDirection::Right(*magnitude),
                'L' => RelativeDirection::Left(*magnitude),
                _ => panic!("Invalid direction"),
            }
        })
        .collect()
}

fn follow_direction(context: &Context, direction: &RelativeDirection) -> Context {
    let coordinates = &context.coordinates;
    match (&context.facing, direction) {
        (Direction::North, RelativeDirection::Right(magnitude)) => Context {
            facing: Direction::East,
            coordinates: Coordinates {
                x: coordinates.x + magnitude,
                y: coordinates.y,
            },
        },
        (Direction::North, RelativeDirection::Left(magnitude)) => Context {
            facing: Direction::West,
            coordinates: Coordinates {
                x: coordinates.x - magnitude,
                y: coordinates.y,
            },
        },
        (Direction::East, RelativeDirection::Right(magnitude)) => Context {
            facing: Direction::South,
            coordinates: Coordinates {
                x: coordinates.x,
                y: coordinates.y - magnitude,
            },
        },
        (Direction::East, RelativeDirection::Left(magnitude)) => Context {
            facing: Direction::North,
            coordinates: Coordinates {
                x: coordinates.x,
                y: coordinates.y + magnitude,
            },
        },
        (Direction::South, RelativeDirection::Right(magnitude)) => Context {
            facing: Direction::West,
            coordinates: Coordinates {
                x: coordinates.x - magnitude,
                y: coordinates.y,
            },
        },
        (Direction::South, RelativeDirection::Left(magnitude)) => Context {
            facing: Direction::East,
            coordinates: Coordinates {
                x: coordinates.x + magnitude,
                y: coordinates.y,
            },
        },
        (Direction::West, RelativeDirection::Right(magnitude)) => Context {
            facing: Direction::North,
            coordinates: Coordinates {
                x: coordinates.x,
                y: coordinates.y + magnitude,
            },
        },
        (Direction::West, RelativeDirection::Left(magnitude)) => Context {
            facing: Direction::South,
            coordinates: Coordinates {
                x: coordinates.x,
                y: coordinates.y - magnitude,
            },
        },
    }
}

fn part_1(input: &str) -> i64 {
    let directions = parse_input(input);
    let mut context = Context {
        facing: Direction::North,
        coordinates: Coordinates { x: 0, y: 0 },
    };
    for direction in &directions {
        context = follow_direction(&context, direction);
    }
    context.coordinates.x + context.coordinates.y
}

fn get_visited(start: &Coordinates, end: &Coordinates) -> Vec<Coordinates> {
    let mut output = vec![];
    if start.x == end.x {
        if start.y < end.y {
            for step in (start.y + 1)..=end.y {
                output.push(Coordinates {
                    x: start.x,
                    y: step,
                });
            }
        } else {
            for step in (end.y..start.y).rev() {
                output.push(Coordinates {
                    x: start.x,
                    y: step,
                });
            }
        }
    } else {
        if start.x < end.x {
            for step in (start.x + 1)..=end.x {
                output.push(Coordinates {
                    x: step,
                    y: start.y,
                });
            }
        } else {
            for step in (end.x..start.x).rev() {
                output.push(Coordinates {
                    x: step,
                    y: start.y,
                });
            }
        }
    }
    output
}

fn part_2(input: &str) -> i64 {
    let directions = parse_input(input);
    let mut visited: HashSet<Coordinates> = HashSet::new();
    let mut context = Context {
        facing: Direction::North,
        coordinates: Coordinates { x: 0, y: 0 },
    };
    visited.insert(context.coordinates.clone());
    for direction in &directions {
        let end_context = follow_direction(&context, direction);
        for step in get_visited(&context.coordinates, &end_context.coordinates) {
            if visited.contains(&step) {
                return step.x.abs() + step.y.abs();
            }
            visited.insert(step);
        }
        context = end_context;
    }
    0
}
