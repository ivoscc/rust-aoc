use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(576, output_part_1);
    // assert_eq!(_, output_part_2);
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

fn get_all_possible_mutations(
    molecule: &String,
    transformations: &HashMap<String, Vec<String>>,
) -> HashSet<String> {
    let mut seen = HashSet::new();
    for (source, possible_transformations) in transformations {
        for (index, source_match) in molecule.match_indices(source) {
            for transformation in possible_transformations.iter() {
                let mutated_molecule =
                    transform_string(&molecule, index, source_match.len(), &transformation);
                seen.insert(mutated_molecule);
            }
        }
    }
    seen
}

type LeveledString = (usize, String);

fn part_2(input: &str) -> usize {
    let (transformations, target_molecule) = parse_input(input);
    let starting_molecule = "e";
    let mut queue: VecDeque<LeveledString> = VecDeque::new();
    queue.push_front((1, starting_molecule.to_string()));
    let mut seen: HashSet<String> = HashSet::new();
    let mut max_len = 0;

    while queue.len() > 0 {
        let (level, molecule) = queue.pop_back().unwrap();
        for mutation in get_all_possible_mutations(&molecule, &transformations) {
            if seen.contains(&mutation) || mutation.len() > target_molecule.len() {
                continue;
            }
            if mutation.len() > max_len {
                max_len = mutation.len();
            }

            if mutation.len() == target_molecule.len() && mutation == target_molecule {
                return level;
            }
            // println!("Mutation = {}", mutation);
            seen.insert(mutation.clone());
            queue.push_front((level + 1, mutation));
            // if seen.len() % 10000 == 0 {
            //     println!(
            //         "Queue is of size {}. Seen strings = {}. Largest {} (target = {})",
            //         queue.len(),
            //         seen.len(),
            //         max_len,
            //         target_molecule.len()
            //     );
            // }
            if queue.len() % 10 == 0 {
                println!(
                    "Queue is of size {}. Largest {} (target = {})",
                    queue.len(),
                    max_len,
                    target_molecule.len()
                );
            }
        }
    }
    0
}
