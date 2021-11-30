use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!("tkspfjcc".to_string(), output_part_1);
    assert_eq!("xrlmbypn".to_string(), output_part_2);
}

fn part_1(input: &str) -> String {
    let mut counters: [HashMap<char, usize>; 8] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    for line in input.lines() {
        line.char_indices().for_each(|(index, character)| {
            *counters[index].entry(character).or_insert(0) += 1;
        })
    }

    let mut output: Vec<char> = vec![];
    for counter in counters.iter() {
        let mut most_common = ' ';
        let mut most_common_count: usize = 0;
        for (character, character_count) in counter {
            if character_count > &most_common_count {
                most_common = *character;
                most_common_count = *character_count;
            }
        }
        output.push(most_common);
    }

    output.iter().collect()
}

fn part_2(input: &str) -> String {
    let mut counters: [HashMap<char, usize>; 8] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    for line in input.lines() {
        line.char_indices().for_each(|(index, character)| {
            *counters[index].entry(character).or_insert(0) += 1;
        })
    }

    let mut output: Vec<char> = vec![];
    for counter in counters.iter() {
        let mut least_common = ' ';
        let mut least_common_count: usize = 0;
        for (character, character_count) in counter {
            if least_common_count == 0 || character_count < &least_common_count {
                least_common = *character;
                least_common_count = *character_count;
            }
        }
        output.push(least_common);
    }

    output.iter().collect()
}
