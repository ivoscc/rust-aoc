use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(339477, output_part_1);
    assert_eq!(3049320156, output_part_2);
}

fn get_first_invalid_character(line: &str) -> Option<char> {
    let mut stack: Vec<char> = vec![];

    for character in line.chars() {
        if character == '(' || character == '[' || character == '{' || character == '<' {
            stack.push(character);
            continue;
        }

        let invalid_character = match (stack.pop(), character) {
            (Some('('), ')') => None,
            (Some('['), ']') => None,
            (Some('{'), '}') => None,
            (Some('<'), '>') => None,
            _ => Some(character),
        };
        if invalid_character.is_some() {
            return invalid_character;
        }
    }

    None
}

fn part_1(input: &str) -> usize {
    let character_score: HashMap<char, usize> =
        HashMap::from_iter([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    input
        .lines()
        .filter_map(|line| match get_first_invalid_character(line) {
            Some(character) => Some(character_score.get(&character).unwrap()),
            None => None,
        })
        .sum()
}

fn get_completion_characters(line: &str) -> Option<Vec<char>> {
    let mut stack: Vec<char> = vec![];

    for character in line.chars() {
        if character == '(' || character == '[' || character == '{' || character == '<' {
            stack.push(character);
            continue;
        }

        let invalid_character = match (stack.pop(), character) {
            (Some('('), ')') => None,
            (Some('['), ']') => None,
            (Some('{'), '}') => None,
            (Some('<'), '>') => None,
            _ => Some(character),
        };
        if invalid_character.is_some() {
            return None;
        }
    }

    let mut completion = vec![];
    for unclosed in stack.iter().rev() {
        completion.push(match unclosed {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Invalid character in stack"),
        });
    }

    Some(completion)
}

fn part_2(input: &str) -> usize {
    let character_score: HashMap<char, usize> =
        HashMap::from_iter([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut scores: Vec<usize> = input
        .lines()
        .filter_map(|line| get_completion_characters(line))
        .map(|completion_characters| {
            let mut score = 0;
            for character in completion_characters {
                score *= 5;
                score += character_score.get(&character).unwrap();
            }
            score
        })
        .collect::<Vec<_>>();
    scores.sort();
    scores[scores.len() / 2]
}
