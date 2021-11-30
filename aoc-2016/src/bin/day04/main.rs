use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(409147, output_part_1);
    assert_eq!(991, output_part_2);
}

fn part_1(input: &str) -> usize {
    let mut output = 0;
    for line in input.lines() {
        let (_, sector_id) = parse_line(line);
        output += sector_id;
    }
    output
}

fn part_2(input: &str) -> usize {
    for line in input.lines() {
        let (line, sector_id) = parse_line(line);
        if sector_id == 0 {
            continue;
        }
        let decoded_room: String = line
            .iter()
            .map(|character| shift_character(character, sector_id))
            .collect();
        if decoded_room == String::from("northpole object storage") {
            return sector_id;
        }
    }
    0
}

fn shift_character(character: &char, sector_id: usize) -> char {
    match character {
        '-' => ' ',
        _ => {
            ((((*character as usize) - ('a' as usize) + sector_id) % 26) + 'a' as usize) as u8
                as char
        }
    }
}

struct Room {
    name: HashMap<char, usize>,
    sector_id: usize,
    checksum: [char; 5],
}

fn split_sector_and_checksum(input: &str) -> (usize, [char; 5]) {
    let parts: Vec<&str> = input.split("[").collect();
    assert!(parts.len() == 2, "Invalid checksum + sector");
    let sector_id = parts[0].parse().expect("Unable to parse sector id");
    let checksum = parts[1]
        .strip_suffix("]")
        .expect("Unexpected checksum format")
        .chars()
        .collect::<Vec<char>>()
        .try_into()
        .expect("Unable to convert checksum.");
    (sector_id, checksum)
}

fn parse_line(line: &str) -> (Vec<char>, usize) {
    let parts: Vec<&str> = line.split("-").collect();
    let line: Vec<char> = parts[..parts.len() - 1].join("-").chars().collect();
    let mut counter: HashMap<char, usize> = HashMap::new();
    line.iter().for_each(|character| {
        *counter.entry(*character).or_insert(0) += 1;
    });
    let (sector_id, checksum) = split_sector_and_checksum(parts.last().unwrap());
    if is_valid_checksum(&counter, checksum) {
        return (line, sector_id);
    }
    (line, 0)
}

fn is_valid_checksum(counter: &HashMap<char, usize>, checksum: [char; 5]) -> bool {
    let mut last_count: usize = 0;
    let mut last_char = 0 as char;
    for character in checksum {
        if let Some(character_count) = counter.get(&character) {
            if last_count == 0
                || *character_count < last_count
                || (*character_count == last_count && last_char < character)
            {
                last_count = *character_count;
                last_char = character;
                continue;
            }
            return false;
        } else {
            return false;
        }
    }
    true
}
