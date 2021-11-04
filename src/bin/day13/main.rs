use itertools::Itertools;
use std::{collections::HashMap, slice::Windows};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(733, output_part_1);
    assert_eq!(725, output_part_2);
}

type HappinessMap = HashMap<String, HashMap<String, i64>>;

fn parse_map(input: &str) -> HappinessMap {
    let mut map: HappinessMap = HashMap::new();
    for line in input.lines() {
        let parts: Vec<&str> = line.split(" ").collect();
        let source_person = parts[0];
        let amount: i64 = match parts[2] {
            "gain" => parts[3].parse::<i64>().unwrap(),
            "lose" => -parts[3].parse::<i64>().unwrap(),
            _ => panic!("Unexpected gain/lose"),
        };
        let destination_person = parts[10].strip_suffix(".").unwrap();

        let inner = map
            .entry(source_person.to_string())
            .or_insert(HashMap::new());
        inner.insert(destination_person.to_string(), amount);
    }
    map
}

fn get_happiness_change_for_setting(setting: &Vec<&String>, map: &HappinessMap) -> i64 {
    let mut happiness_change = 0;
    let first = setting[0].to_owned();
    let second = setting[1].to_owned();
    let one_to_last = setting[setting.len() - 2].to_owned();
    let last = setting[setting.len() - 1].to_owned();

    let first_last_pair = vec![&last, &first, &second];
    let last_first_pair = vec![&one_to_last, &last, &first];

    for pair in setting
        .windows(3)
        .into_iter()
        .chain(first_last_pair.windows(3))
        .chain(last_first_pair.windows(3))
    {
        let person_0 = pair[0];
        let person_1 = pair[1];
        let person_2 = pair[2];

        happiness_change += map
            .get(person_1)
            .get_or_insert(&HashMap::new())
            .get(person_0)
            .unwrap_or(&0);

        happiness_change += map
            .get(person_1)
            .get_or_insert(&HashMap::new())
            .get(person_2)
            .unwrap_or(&0);
    }
    happiness_change
}

fn maximize_happiness(all_names: &Vec<String>, map: HappinessMap) -> i64 {
    let permutations = all_names.iter().permutations(all_names.len());
    let mut max_happiness = 0;
    for permutation in permutations {
        let setting_happiness = get_happiness_change_for_setting(&permutation, &map);
        if setting_happiness > max_happiness {
            max_happiness = setting_happiness;
        }
    }
    max_happiness
}

fn part_1(input: &str) -> i64 {
    let map = parse_map(input);
    let all_names: Vec<String> = map.keys().map(|name| name.to_string()).collect();
    let permutations = all_names.iter().permutations(all_names.len());
    maximize_happiness(&all_names, map)
}

fn part_2(input: &str) -> i64 {
    let mut map = parse_map(input);
    let self_name = String::from("me");

    let mut all_names: Vec<String> = map.keys().map(|name| name.to_string()).collect();

    for name in &all_names {
        map.get_mut(&name.clone())
            .unwrap()
            .insert(self_name.clone(), 0);
        map.entry(self_name.clone())
            .or_insert(HashMap::new())
            .insert(name.to_string(), 0);
    }
    all_names.push(self_name);

    let permutations = all_names.iter().permutations(all_names.len());
    maximize_happiness(&all_names, map)
}
