use std::{
    collections::HashSet,
    iter::{self, Chain},
    slice::Iter,
};

use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    // assert_eq!(_, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn parse_input(input: &str) -> Result<Vec<i64>, String> {
    let mut output = vec![];
    for line in input.lines() {
        output.push(
            line.parse()
                .map_err(|_| format!("Can't parse line {}", line))?,
        )
    }
    Ok(output)
}

fn generate_groups(target: i64, weights: &[i64]) -> HashSet<Vec<i64>> {
    let mut output = HashSet::new();
    let mut current_path = vec![];
    generate_groups_helper(target, weights, &mut current_path, &mut output);
    output
}

fn generate_groups_helper(
    target: i64,
    weights: &[i64],
    current_path: &mut Vec<i64>,
    output: &mut HashSet<Vec<i64>>,
) -> () {
    if target == 0 {
        output.insert(current_path.clone());
    }

    if target < 0 || weights.len() == 0 {
        return;
    }

    let number = weights[0];

    current_path.push(number);
    generate_groups_helper(target - number, &weights[1..], current_path, output);
    current_path.pop();
    generate_groups_helper(target, &weights[1..], current_path, output);
}

fn part_1(input: &str) -> i64 {
    // all groups must weight the same
    // first group needs as few packages as possible
    // QE of a group is product of the weights (only to break tie )
    // let input = "1\n2\n3\n4\n5\n7\n8\n9\n10\n11";
    let weights = parse_input(input).unwrap();
    let target_weight_per_group = weights.iter().sum::<i64>() / 3;
    let output = generate_groups(target_weight_per_group, &weights);
    println!("Output = {}", output.len());
    for group_size in 1..weights.len() {
        let mut smallest_qe = 0;
        for group in output.iter().filter(|group| group.len() == group_size) {
            let group_qe: i64 = group.iter().product();
            if smallest_qe == 0 || group_qe < smallest_qe {
                println!("group = {:?}", group);
                smallest_qe = group_qe;
            }
        }
        if smallest_qe != 0 {
            return smallest_qe;
        }
    }
    0
}

fn part_2(input: &str) -> i64 {
    let weights = parse_input(input).unwrap();
    let target_weight_per_group = weights.iter().sum::<i64>() / 4;
    let output = generate_groups(target_weight_per_group, &weights);
    println!("Output = {}", output.len());
    for group_size in 1..weights.len() {
        let mut smallest_qe = 0;
        for group in output.iter().filter(|group| group.len() == group_size) {
            let group_qe: i64 = group.iter().product();
            if smallest_qe == 0 || group_qe < smallest_qe {
                println!("group = {:?}", group);
                smallest_qe = group_qe;
            }
        }
        if smallest_qe != 0 {
            return smallest_qe;
        }
    }
    0
}
