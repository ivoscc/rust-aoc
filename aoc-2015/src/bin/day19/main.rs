use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use regex::{Captures, Regex};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(576, output_part_1);
    assert_eq!(207, output_part_2);
}

fn parse_input(input: &str) -> (HashMap<String, Vec<String>>, String) {
    let mut transformations = HashMap::new();
    let mut molecule: String = String::from("");
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(" ").collect();
        if line_parts.len() == 3 {
            transformations
                .entry(line_parts[0].to_string())
                .or_insert(vec![])
                .push(line_parts[2].to_string());
        }
        if line_parts.len() == 1 {
            molecule = String::from(line_parts[0]);
        }
    }
    (transformations, molecule)
}

fn transform_string(string: &str, index: usize, num_chars: usize, replacement: &str) -> String {
    let (first_part, rest) = string.split_at(index);
    first_part
        .chars()
        .chain(replacement.chars())
        .chain(rest.chars().skip(num_chars))
        .collect()
}

fn part_1(input: &str) -> usize {
    let (transformations, molecule) = parse_input(input);
    let mut seen: HashSet<String> = HashSet::new();
    let molecule: String = molecule;
    for (source, possible_transformations) in transformations {
        for transformation in possible_transformations.iter() {
            for (index, source_match) in molecule.match_indices(&source) {
                let mutated_molecule =
                    transform_string(&molecule, index, source_match.len(), &transformation);
                seen.insert(mutated_molecule);
            }
        }
    }
    seen.len()
}

fn parse_input_into_reverse_map(input: &str) -> (HashMap<String, String>, String) {
    let mut map = HashMap::new();
    let mut molecule = String::from("");
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(" ").collect();

        if line_parts.len() == 0 {
            continue;
        }
        if line_parts.len() == 3 {
            map.insert(line_parts[2].to_string(), line_parts[0].to_string());
        } else {
            molecule = line.to_string();
        }
    }
    (map, molecule)
}

// BFS was too slow here
fn part_2(input: &str) -> usize {
    let (transformations, starting_molecule) = parse_input_into_reverse_map(input);
    let target_molecule = "e";
    let mut molecule: String = starting_molecule.clone();
    let mut steps = 0;
    let regex: String = transformations.keys().join("|");
    let re: Regex = Regex::new(&regex).unwrap();

    while molecule != target_molecule {
        molecule = re
            .replace_all(&molecule, |caps: &Captures| {
                steps += 1;
                transformations.get(&caps[0]).unwrap()
            })
            .to_string();
    }

    steps
}
