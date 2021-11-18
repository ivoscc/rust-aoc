use std::{collections::HashMap, io::BufRead};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(2640, output_part_1);
    assert_eq!(1102, output_part_2);
}

#[derive(Debug)]
struct ReindeerStats {
    speed: usize,
    sprint_duration: usize,
    rest_duration: usize,
}

type ReindeerStatsMap = HashMap<String, ReindeerStats>;

fn parse_speeds(input: &str) -> HashMap<String, ReindeerStats> {
    let mut map: ReindeerStatsMap = HashMap::new();
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(" ").collect();
        let name = line_parts[0];
        let speed = line_parts[3].parse::<usize>().unwrap();
        let sprint_duration = line_parts[6].parse::<usize>().unwrap();
        let rest_duration = line_parts[13].parse::<usize>().unwrap();
        map.insert(
            name.to_string(),
            ReindeerStats {
                speed,
                sprint_duration,
                rest_duration,
            },
        );
    }
    map
}

fn get_distance_after_n_seconds(total_seconds: usize, stats: &ReindeerStats) -> usize {
    let segment_duration = stats.sprint_duration + stats.rest_duration;
    let completed_segments: usize = total_seconds / segment_duration;
    let full_segments_duration = segment_duration * completed_segments;
    let last_segment_duration = total_seconds - full_segments_duration;
    let total_distance_in_full_sprints = completed_segments * stats.speed * stats.sprint_duration;
    let distance_in_last_sprint = if last_segment_duration > stats.sprint_duration {
        stats.speed * stats.sprint_duration
    } else {
        stats.speed * last_segment_duration
    };
    total_distance_in_full_sprints + distance_in_last_sprint
}

fn part_1(input: &str) -> usize {
    let map = parse_speeds(input);
    let total_seconds = 2503;
    let mut max_distance = 0;
    for (reindeer_name, stats) in &map {
        let current_distance = get_distance_after_n_seconds(total_seconds, stats);
        if current_distance > max_distance {
            max_distance = current_distance
        }
    }
    max_distance
}

fn part_2(input: &str) -> usize {
    let map = parse_speeds(input);
    let total_seconds = 2503;

    let mut scoreboard: HashMap<String, usize> = HashMap::new();
    for step in 1..=total_seconds {
        let mut distances_map: HashMap<usize, Vec<String>> = HashMap::new();
        let mut max_distance = 0;
        for (reindeer_name, stats) in &map {
            let current_distance = get_distance_after_n_seconds(step, &stats);
            distances_map
                .entry(current_distance)
                .or_insert(Vec::<String>::new())
                .push(reindeer_name.to_string());
            if current_distance > max_distance {
                max_distance = current_distance;
            }
        }
        for reindeer_name in distances_map.get(&max_distance).unwrap() {
            let entry = scoreboard.entry(reindeer_name.to_string()).or_insert(0);
            *entry += 1;
        }
    }

    let mut max_score = 0;
    for (_, score) in scoreboard {
        if score > max_score {
            max_score = score;
        }
    }

    max_score
}
