use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(373, output_part_1);
    assert_eq!(260, output_part_2);
}

fn parse_aunt_list(input: &str) -> Vec<HashMap<String, usize>> {
    let mut aunts = vec![];
    for line in input.lines() {
        let mut aunt_properties: HashMap<String, usize> = HashMap::new();
        let line_parts: Vec<&str> = line.split(" ").into_iter().skip(2).collect();
        for chunk in line_parts.chunks(2) {
            let property_name = chunk[0].strip_suffix(":").unwrap();
            let property_value = (if chunk[1].ends_with(",") {
                chunk[1].strip_suffix(",").unwrap()
            } else {
                chunk[1]
            })
            .parse::<usize>()
            .unwrap();
            aunt_properties.insert(property_name.to_string(), property_value);
        }
        aunts.push(aunt_properties);
    }
    aunts
}

fn get_target_properties() -> HashMap<String, usize> {
    let mut properties = HashMap::new();
    properties.insert(String::from("children"), 3);
    properties.insert(String::from("cats"), 7);
    properties.insert(String::from("samoyeds"), 2);
    properties.insert(String::from("pomeranians"), 3);
    properties.insert(String::from("akitas"), 0);
    properties.insert(String::from("vizslas"), 0);
    properties.insert(String::from("goldfish"), 5);
    properties.insert(String::from("trees"), 3);
    properties.insert(String::from("cars"), 2);
    properties.insert(String::from("perfumes"), 1);
    properties
}

fn part_1(input: &str) -> usize {
    let target_properties = get_target_properties();
    let aunts = parse_aunt_list(input);
    let mut matching_aunts: Vec<usize> = vec![];
    for (aunt_index, aunt) in aunts.iter().enumerate() {
        let mut matches = true;
        for (property, value) in aunt {
            if let Some(target_value) = target_properties.get(property) {
                if target_value != value {
                    matches = false;
                    break;
                }
            }
        }
        if matches {
            matching_aunts.push(aunt_index + 1);
        }
    }
    matching_aunts[0]
}

fn part_2(input: &str) -> usize {
    let target_properties = get_target_properties();
    let aunts = parse_aunt_list(input);
    let mut matching_aunts: Vec<usize> = vec![];
    for (aunt_index, aunt) in aunts.iter().enumerate() {
        let mut matches = true;
        for (property, value) in aunt {
            if let Some(target_value) = target_properties.get(property) {
                match &property[..] {
                    "cats" | "trees" => {
                        if value <= target_value {
                            matches = false;
                            break;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if value >= target_value {
                            matches = false;
                            break;
                        }
                    }
                    _ => {
                        if target_value != value {
                            matches = false;
                            break;
                        }
                    }
                };
            }
        }
        if matches {
            matching_aunts.push(aunt_index + 1);
        }
    }
    matching_aunts[0]
}
