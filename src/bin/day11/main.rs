fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    // assert_eq!(_, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn is_valid_password(input: &str) -> bool {
    false
}

fn next_password(password: &str) -> String {
    if password.len() == 0 {
        return String::from("");
    }
    let mut output: Vec<char> = vec![];

    let min_bytechar: u8 = 'a' as u8;
    let max_bytechar: u8 = 'z' as u8;

    let mut bytes_iterator = password.bytes().rev();
    let mut remainder: u8 = 1;

    while remainder > 0 && output.len() < 8 {
        if let Some(bytechar) = bytes_iterator.next() {
            let next_bytechar = bytechar + remainder;
            remainder = if next_bytechar >= max_bytechar {
                next_bytechar - max_bytechar
            } else {
                0
            };
            if remainder > 0 {
                output.push(min_bytechar as char);
            } else {
                output.push(next_bytechar as char)
            }
        } else {
            let next_bytechar = min_bytechar + remainder;
            remainder = if next_bytechar >= max_bytechar {
                next_bytechar - max_bytechar
            } else {
                0
            };
        }
        if remainder > 0 {
            output.push(min_bytechar as char);
        } else {
            output.push(next_bytechar as char)
        }
    }

    while let Some(byte_char) = bytes_iterator.next() {
        output.push(byte_char as char)
    }

    output.into_iter().take(8).rev().collect()
}

fn part_1(input: &str) -> String {
    let input = "zzzzz";
    let mut password: String = input.trim().into();
    // password = next_password(&password);
    println!("Password is {}", password);
    for _ in 0..4 {
        password = next_password(&password);
        println!("Next password is {}", password);
    }
    // loop {
    //     password = next_password(&password);
    //     // if is_valid_password(&password) {
    //     //     break;
    //     // }
    // }
    password
}

fn part_2(input: &str) -> () {}
