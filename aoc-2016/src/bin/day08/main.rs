use std::fmt;

const SCREEN_WIDTH: usize = 50;
const SCREEN_HEIGHT: usize = 6;

struct Screen([[bool; SCREEN_WIDTH]; SCREEN_HEIGHT]);

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut lines: Vec<String> = vec![];
        for row in self.0 {
            let mut line = vec![];
            for light in row {
                if light {
                    line.push('#');
                } else {
                    line.push(' ');
                }
            }
            lines.push(line.into_iter().collect());
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl Screen {
    fn count_lit(&self) -> usize {
        let mut count = 0;
        for row in self.0 {
            for light in row {
                if light {
                    count += 1;
                }
            }
        }
        count
    }
}

#[derive(Debug)]
enum Instruction {
    Rect(u8, u8),
    RotateRow(u8, u8),
    RotateCol(u8, u8),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = input.split(" ").collect::<Vec<&str>>();
        if parts.len() == 2 {
            let dimensions = parts[1]
                .split("x")
                .map(|s| s.parse::<u8>().expect("Can't parse item."))
                .collect::<Vec<u8>>();
            return Ok(Instruction::Rect(dimensions[0], dimensions[1]));
        }
        match parts[1] {
            "row" => {
                let target_row = parts[2]
                    .split("=")
                    .last()
                    .map(|s| s.parse::<u8>().expect("Cant parse item."))
                    .ok_or("Invalid target row")?;
                Ok(Instruction::RotateRow(
                    target_row,
                    parts[4].parse::<u8>().expect("Invalid shift"),
                ))
            }
            "column" => {
                let target_col = parts[2]
                    .split("=")
                    .last()
                    .map(|s| s.parse::<u8>().expect("Cant parse item."))
                    .ok_or("Invalid target col")?;
                Ok(Instruction::RotateCol(
                    target_col,
                    parts[4].parse::<u8>().expect("Invalid shift"),
                ))
            }
            _ => panic!("Unknown type"),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(115, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn apply_rect(screen: &mut Screen, height: usize, width: usize) {
    let Screen(screen) = screen;
    for col in 0..width {
        for row in 0..height {
            screen[row][col] = true;
        }
    }
}

fn apply_row_rotation(screen: &mut Screen, row: usize, shift: usize) {
    let Screen(screen) = screen;
    let row_size = screen[row].len();
    let shift = shift % row_size;
    if shift == row_size {
        return;
    }
    let mut buffer: Vec<bool> = vec![];
    for index in (0..row_size).rev() {
        if index >= row_size - shift {
            buffer.push(screen[row][index]);
        }
        if index + shift < row_size {
            screen[row][index + shift] = screen[row][index];
        }
        if index < shift {
            screen[row][index] = buffer[shift - 1 - index];
        }
    }
}

fn apply_col_rotation(screen: &mut Screen, col: usize, shift: usize) {
    let Screen(screen) = screen;
    let col_size = screen.len();
    let shift = shift % col_size;
    if shift == col_size {
        return;
    }
    let mut buffer: Vec<bool> = vec![];
    for index in (0..col_size).rev() {
        if index >= col_size - shift {
            buffer.push(screen[index][col]);
        }
        if index + shift < col_size {
            screen[index + shift][col] = screen[index][col]
        }
        if index < shift {
            screen[index][col] = buffer[shift - 1 - index];
        }
    }
}

fn apply_instruction(screen: &mut Screen, instruction: &Instruction) {
    match *instruction {
        Instruction::Rect(width, height) => apply_rect(screen, height as usize, width as usize),
        Instruction::RotateRow(row, shift) => {
            apply_row_rotation(screen, row as usize, shift as usize)
        }
        Instruction::RotateCol(col, shift) => {
            apply_col_rotation(screen, col as usize, shift as usize)
        }
    }
}

fn part_1(input: &str) -> usize {
    let mut screen = Screen([[false; SCREEN_WIDTH]; SCREEN_HEIGHT]);
    for line in input.lines() {
        let instruction: Instruction = line.try_into().expect("Unable to parse instruction.");
        apply_instruction(&mut screen, &instruction)
    }
    screen.count_lit()
}

fn part_2(input: &str) -> () {
    let mut screen = Screen([[false; SCREEN_WIDTH]; SCREEN_HEIGHT]);
    for line in input.lines() {
        let instruction: Instruction = line.try_into().expect("Unable to parse instruction.");
        apply_instruction(&mut screen, &instruction)
    }
    println!("{}", screen);
}
