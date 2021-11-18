use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(&output_part_1);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!("hepxxyzz", output_part_1);
    assert_eq!("heqaabcc", output_part_2);
}

fn is_valid_password(
    input: &str,
    invalid_characters: &HashSet<char>,
    repeats: &HashSet<[char; 2]>,
    triplets: &HashSet<[char; 3]>,
) -> bool {
    for character in input.chars() {
        if invalid_characters.contains(&character) {
            return false;
        }
    }

    let mut contains_triplet = false;
    for triplet in input.chars().collect::<Vec<char>>().windows(3) {
        if triplets.contains(triplet) {
            contains_triplet = true;
            break;
        }
    }

    let mut contains_repeated_pair = 0;
    let mut last_repeat = false;
    for pair in input.chars().collect::<Vec<char>>().windows(2) {
        if repeats.contains(pair) && !last_repeat {
            contains_repeated_pair += 1;
            last_repeat = true;
        } else {
            last_repeat = false;
        }
    }

    contains_triplet && (contains_repeated_pair > 1)
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
        let sum_bytechar = if let Some(bytechar) = bytes_iterator.next() {
            bytechar + remainder
        } else {
            min_bytechar + remainder
        };
        remainder = if sum_bytechar >= max_bytechar {
            sum_bytechar - max_bytechar
        } else {
            0
        };

        if remainder > 0 {
            output.push(min_bytechar as char);
        } else {
            output.push(sum_bytechar as char)
        }
    }

    while let Some(byte_char) = bytes_iterator.next() {
        output.push(byte_char as char)
    }

    output.into_iter().take(8).rev().collect()
}

fn part_1(input: &str) -> String {
    let repeated_pairs: HashSet<[char; 2]> =
        HashSet::from_iter(('a'..='z').map(|character| [character, character]));

    let ascending_triplets: HashSet<[char; 3]> = HashSet::from_iter(
        ('a'..='z')
            .collect::<Vec<char>>()
            .windows(3)
            .map(|triplet| [triplet[0], triplet[1], triplet[2]]),
    );

    let invalid_characters: HashSet<char> = HashSet::from_iter(['i', 'l', 'o'].into_iter());

    let mut password: String = input.trim().into();

    loop {
        password = next_password(&password);
        if is_valid_password(
            &password,
            &invalid_characters,
            &repeated_pairs,
            &ascending_triplets,
        ) {
            break;
        }
    }

    password
}

fn part_2(input: &str) -> String {
    part_1(input)
}
