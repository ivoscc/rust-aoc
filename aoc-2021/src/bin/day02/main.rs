struct Position {
    depth: i32,
    width: i32,
    aim: i32,
}
enum Instruction {
    Down(i32),
    Up(i32),
    Forward(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        let parts = string.split(" ").collect::<Vec<_>>();
        let magnitude = parts[1].parse::<i32>().expect("Magniture parse error");

        match parts[0] {
            "forward" => Ok(Self::Forward(magnitude)),
            "down" => Ok(Self::Down(magnitude)),
            "up" => Ok(Self::Up(magnitude)),
            _ => panic!("Unknown instruction"),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1383564, output_part_1);
    assert_eq!(1488311643, output_part_2);
}

fn part_1(input: &str) -> i32 {
    let mut position: (i32, i32) = (0, 0);
    for line in input.lines() {
        let instruction: Instruction = line.try_into().expect("Unable to parse instruction");
        match instruction {
            Instruction::Forward(magnitude) => {
                position = (position.0, position.1 + magnitude);
            }
            Instruction::Down(magnitude) => {
                position = (position.0 - magnitude, position.1);
            }
            Instruction::Up(magnitude) => {
                position = (position.0 + magnitude, position.1);
            }
        }
    }
    position.0.abs() * (position.1.abs())
}

fn part_2(input: &str) -> i32 {
    let mut position: (i32, i32, i32) = (0, 0, 0);
    for line in input.lines() {
        let instruction: Instruction = line.try_into().expect("Unable to parse instruction");
        match instruction {
            Instruction::Forward(magnitude) => {
                position = (
                    position.0 + magnitude,
                    position.1 + (position.2 * magnitude),
                    position.2,
                );
            }
            Instruction::Down(magnitude) => {
                position = (position.0, position.1, position.2 + magnitude);
            }
            Instruction::Up(magnitude) => {
                position = (position.0, position.1, position.2 - magnitude);
            }
        }
    }
    position.0.abs() * (position.1.abs())
}
