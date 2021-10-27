use std::collections::HashSet;

fn is_nice(string: &str) -> bool {
    let mut vowel_counter = 0;
    let mut has_repeated = false;
    let mut last_char: Option<char> = None;

    let mut vowels = HashSet::new();
    vowels.extend(['a', 'e', 'i', 'o', 'u']);

    let mut bad_words = HashSet::new();
    bad_words.extend(["ab", "cd", "pq", "xy"]);

    for character in string.chars() {
        if vowels.contains(&character) {
            vowel_counter += 1;
        }
        if let Some(last_char) = last_char {
            let combo = format!("{}{}", last_char, character);
            if bad_words.contains(&combo[..]) {
                return false;
            }
            if character == last_char {
                has_repeated = true;
            }
        }
        last_char = Some(character);
    }
    vowel_counter >= 3 && has_repeated
}

fn part_1(input: &str) -> usize {
    input.lines()
         .filter(|line| is_nice(line)).count()
}

// fn is_nice_2(string: &str) -> bool {
//     let string: Vec<char> = string.chars().collect();
//     let mut contains_character_sandwich = false;
//     for triplet in string.windows(3) {
//         if triplet[0] == triplet[2] {
//             contains_character_sandwich = true;
//             break;
//         }
//     }

//     let mut seen_pairs: HashSet<String> = HashSet::new();
//     let mut last_pair: Option<String> = None;
//     let mut has_repeated_pairs = false;
//     for uno in string.windows(2) {
//         let pair: String = format!("{}{}", uno[0], uno[1]);
//         if seen_pairs.contains(&pair) && (pair != last_pair.unwrap()) {
//             has_repeated_pairs = true;
//             break
//         }
//         seen_pairs.insert(String::from(pair.clone()));
//         last_pair = Some(String::from(pair.clone()));
//     }
//     println!("{}, {}",contains_character_sandwich, has_repeated_pairs);
//     contains_character_sandwich && has_repeated_pairs
// }
fn is_nice_2(string: &str) -> bool {
    let string: Vec<char> = string.chars().collect();
    let mut contains_character_sandwich = false;
    for triplet in string.windows(3) {
        if triplet[0] == triplet[2] {
            contains_character_sandwich = true;
            break;
        }
    }

    let mut seen_pairs: HashSet<String> = HashSet::new();
    let mut last_pair: Option<String> = None;
    let mut has_repeated_pairs = false;
    let mut last_pair_times = 0;
    for uno in string.windows(2) {
        let pair: String = format!("{}{}", uno[0], uno[1]);
        if seen_pairs.contains(&pair) && ((&pair[..] != &last_pair.clone().unwrap()) || last_pair_times > 1) {
            has_repeated_pairs = true;
            break
        }
        seen_pairs.insert(String::from(pair.clone()));
        if pair == last_pair.unwrap_or_default() {
            last_pair_times += 1;
        } else {
            last_pair_times = 1;
        }
        last_pair = Some(String::from(pair.clone()));
    }
    contains_character_sandwich && has_repeated_pairs
}

fn part_2(input: &str) -> usize {
    input.lines()
         .filter(|line| is_nice_2(line)).count()
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(236, output_part_1);
    assert_eq!(51, output_part_2);
}
