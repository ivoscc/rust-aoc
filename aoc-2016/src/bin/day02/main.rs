fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!("61529", output_part_1);
    assert_eq!("C2C28", output_part_2);
}

enum Instruction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<char> for Instruction {
    type Error = &'static str;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'U' => Ok(Self::Up),
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            _ => Err("Invalid character"),
        }
    }
}

fn parse_instructions(input: &str) -> Vec<Vec<Instruction>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|character| character.try_into().expect("Failed to parse input."))
                .collect()
        })
        .collect()
}

struct Position {
    row: i8,
    col: i8,
}

fn update_position(position: &mut Position, instruction: &Instruction, keypad: &Keypad) -> () {
    let (new_row, new_col) = match instruction {
        Instruction::Up => (position.row - 1, position.col),
        Instruction::Right => (position.row, position.col + 1),
        Instruction::Down => (position.row + 1, position.col),
        Instruction::Left => (position.row, position.col - 1),
    };
    if new_row < 0
        || new_col < 0
        || new_row >= keypad.len() as i8
        || new_col >= keypad[0].len() as i8
        || keypad[new_row as usize][new_col as usize] == '#'
    {
        return;
    }

    position.col = new_col;
    position.row = new_row;
}

fn apply_instructions(
    start: &mut Position,
    instructions: &Vec<Instruction>,
    keypad: &Keypad,
) -> () {
    for instruction in instructions {
        update_position(start, instruction, keypad)
    }
}

type Keypad = Vec<Vec<char>>;

fn part_1(input: &str) -> String {
    let instructions_set_list = parse_instructions(input);
    let keypad = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    let mut position = Position { row: 1, col: 1 };
    let mut output: Vec<char> = vec![];

    for instruction_set in &instructions_set_list {
        apply_instructions(&mut position, instruction_set, &keypad);
        output.push(keypad[position.row as usize][position.col as usize]);
    }
    output.iter().collect()
}

fn part_2(input: &str) -> String {
    let instructions_set_list = parse_instructions(input);
    let keypad = vec![
        vec!['#', '#', '1', '#', '#'],
        vec!['#', '2', '3', '4', '#'],
        vec!['5', '6', '7', '8', '9'],
        vec!['#', 'A', 'B', 'C', '#'],
        vec!['#', '#', 'D', '#', '#'],
    ];
    let mut position = Position { row: 2, col: 0 };
    let mut output: Vec<char> = vec![];

    for instruction_set in &instructions_set_list {
        apply_instructions(&mut position, instruction_set, &keypad);
        output.push(keypad[position.row as usize][position.col as usize]);
    }
    output.iter().collect()
}
