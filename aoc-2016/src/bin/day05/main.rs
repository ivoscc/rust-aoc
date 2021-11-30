use crypto::{digest::Digest, md5::Md5};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!("c6697b55".to_string(), output_part_1);
    assert_eq!("8c35d1ab".to_string(), output_part_2);
}

fn part_1(input: &str) -> String {
    let door = input.trim();
    generate_password(door, 8)
}

fn generate_password(door: &str, password_length: usize) -> String {
    let mut password: Vec<char> = vec![];
    let mut index: usize = 0;
    let mut hasher = Md5::new();
    while password.len() < password_length {
        hasher.input_str(&[door, &index.to_string()].concat());
        if hasher.result_str().starts_with("00000") {
            password.push(hasher.result_str().chars().skip(5).next().unwrap());
        }
        hasher.reset();
        index += 1;
    }
    password.iter().collect()
}

fn part_2(input: &str) -> String {
    let door = input.trim();
    let mut password: [char; 8] = ['_'; 8];
    let mut found_chars = 0;
    let mut index: usize = 0;
    let mut hasher = Md5::new();
    let mut out: [u8; 16] = [0; 16];
    while found_chars < 8 {
        hasher.input_str(&[door, &index.to_string()].concat());
        hasher.result(&mut out);
        if out[0] == 0 && out[1] == 0 && (out[2] & 0xF0 == 0) {
            let position = (out[2] & 0x0F) as usize;
            if position < 8 && password[position] == '_' {
                password[position] = char::from_digit(((out[3] & 0xF0) >> 4).into(), 16).unwrap();
                found_chars += 1;
            }
        }
        hasher.reset();
        index += 1;
    }
    password.iter().collect()
}
