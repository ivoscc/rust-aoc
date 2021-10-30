use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(207, output_part_1);
    assert_eq!(804, output_part_2);
}

type DistancesMap = HashMap<String, HashMap<String, usize>>;

fn parse_distance_map(input: &str) -> DistancesMap {
    let mut distances_map: DistancesMap = HashMap::new();
    input.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        let start = parts[0];
        let end = parts[2];
        let distance: usize = parts[4].parse::<usize>().unwrap();

        // Store the distance from `start` to `end`
        let inner = distances_map
            .entry(start.to_string())
            .or_insert(HashMap::new());
        inner.insert(end.to_string(), distance);

        // Store the distance from `end` to `start`
        let inner = distances_map
            .entry(end.to_string())
            .or_insert(HashMap::new());
        inner.insert(start.to_string(), distance);
    });
    distances_map
}

fn get_max_distance_for_starting_point(
    start: &str,
    distances_map: DistancesMap,
    visited: Option<HashSet<String>>,
) -> usize {
    let mut visited = visited.unwrap_or(HashSet::new());
    visited.insert(start.to_string());
    let mut max_distance: Option<usize> = None;
    for (next_city, distance_to_next_city) in
        distances_map.get(&start[..]).unwrap_or(&HashMap::new())
    {
        if visited.contains(&next_city[..]) {
            continue;
        }
        let path_distance = get_max_distance_for_starting_point(
            &next_city,
            distances_map.clone(),
            Some(visited.clone()),
        );
        max_distance = Some(if let Some(current_max) = max_distance {
            std::cmp::max(current_max, path_distance + distance_to_next_city)
        } else {
            path_distance + distance_to_next_city
        });
    }
    max_distance.unwrap_or(0)
}
fn get_min_distance_for_starting_point(
    start: &str,
    distances_map: DistancesMap,
    visited: Option<HashSet<String>>,
) -> usize {
    let mut visited = visited.unwrap_or(HashSet::new());
    visited.insert(start.to_string());
    let mut min_distance: Option<usize> = None;
    for (next_city, distance_to_next_city) in
        distances_map.get(&start[..]).unwrap_or(&HashMap::new())
    {
        if visited.contains(&next_city[..]) {
            continue;
        }
        let path_distance = get_min_distance_for_starting_point(
            &next_city,
            distances_map.clone(),
            Some(visited.clone()),
        );
        min_distance = Some(if let Some(current_min) = min_distance {
            std::cmp::min(current_min, path_distance + distance_to_next_city)
        } else {
            path_distance + distance_to_next_city
        });
    }
    min_distance.unwrap_or(0)
}

fn part_1(input: &str) -> usize {
    let distances_map = parse_distance_map(input);
    let mut min_distance: Option<usize> = None;
    for (next_city, _) in &distances_map {
        let path_distance =
            get_min_distance_for_starting_point(&next_city, distances_map.clone(), None);
        min_distance = Some(if let Some(min_distance) = min_distance {
            std::cmp::min(min_distance, path_distance)
        } else {
            path_distance
        });
    }
    min_distance.unwrap()
}

fn part_2(input: &str) -> usize {
    let distances_map = parse_distance_map(input);
    let mut max_distance: Option<usize> = None;
    for (next_city, _) in &distances_map {
        let path_distance =
            get_max_distance_for_starting_point(&next_city, distances_map.clone(), None);
        max_distance = Some(if let Some(max_distance) = max_distance {
            std::cmp::max(max_distance, path_distance)
        } else {
            path_distance
        });
    }
    max_distance.unwrap()
}
