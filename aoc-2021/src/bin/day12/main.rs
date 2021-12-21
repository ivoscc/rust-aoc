use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
};

use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Cave {
    Start,
    Small(String),
    Large(String),
    End,
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Cave::Start => "start",
            Cave::End => "end",
            Cave::Small(s) | Cave::Large(s) => s.as_ref(),
        };
        write!(f, "{}", name)
    }
}

fn print_path(path: &Vec<Cave>) {
    let s = path.iter().map(|x| format!("{}", x)).join(",");
    println!("{}", s);
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        match value {
            "start" => Cave::Start,
            "end" => Cave::End,
            val if val.chars().filter(|c| c.is_lowercase()).count() > 0 => {
                Cave::Small(value.to_string())
            }
            _ => Cave::Large(value.to_string()),
        }
    }
}

fn parse_input(input: &str) -> HashMap<Cave, Vec<Cave>> {
    let mut cave_map = HashMap::new();
    input.lines().for_each(|line| {
        let parts = line.split("-").collect::<Vec<_>>();
        let source: Cave = parts[0].into();
        let destination: Cave = parts[1].into();
        cave_map
            .entry(source.clone())
            .or_insert(vec![])
            .push(destination.clone());
        cave_map.entry(destination).or_insert(vec![]).push(source);
    });
    cave_map
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(5076, output_part_1);
    assert_eq!(145643, output_part_2);
}

fn part_1(input: &str) -> usize {
    // let input = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";
    let cave_map = parse_input(input);
    let mut queue = VecDeque::from_iter([(Cave::Start, vec![])]);
    let mut path_count = 0;
    let empty: Vec<Cave> = vec![];

    while queue.len() > 0 {
        let (cave, current_path) = queue.pop_back().unwrap();
        for next_cave in cave_map.get(&cave).unwrap_or(&empty) {
            match next_cave {
                Cave::End => {
                    path_count += 1;
                }
                Cave::Large(_) => {
                    let mut new_path = current_path.clone();
                    new_path.push(next_cave);
                    queue.push_front((next_cave.clone(), new_path));
                }
                Cave::Small(_) => {
                    if current_path.contains(&next_cave) {
                        continue;
                    }
                    let mut new_path = current_path.clone();
                    new_path.push(next_cave);
                    queue.push_front((next_cave.clone(), new_path));
                }
                _ => {}
            }
        }
    }
    path_count
}

fn part_2(input: &str) -> usize {
    let cave_map = parse_input(input);
    let mut path_count = 0;
    let empty: Vec<Cave> = vec![];
    let small_cave_visit_counter: HashMap<Cave, usize> = HashMap::new();
    let mut queue = VecDeque::from_iter([(Cave::Start, small_cave_visit_counter)]);

    while queue.len() > 0 {
        let (cave, current_small_cave_visit_counter) = queue.pop_back().unwrap();
        for next_cave in cave_map.get(&cave).unwrap_or(&empty) {
            match next_cave {
                Cave::End => {
                    path_count += 1;
                }
                Cave::Large(_) => {
                    queue.push_front((next_cave.clone(), current_small_cave_visit_counter.clone()));
                }
                Cave::Small(_) => {
                    let has_visited_small_cave_twice = current_small_cave_visit_counter
                        .values()
                        .filter(|count| **count > 1)
                        .count()
                        > 0;
                    let next_cave_visit_count = current_small_cave_visit_counter
                        .get(&next_cave)
                        .unwrap_or(&0);
                    if *next_cave_visit_count == 0
                        || (!has_visited_small_cave_twice && *next_cave_visit_count == 1)
                    {
                        let mut new_counter = current_small_cave_visit_counter.clone();
                        *new_counter.entry(next_cave.clone()).or_insert(0) += 1;
                        queue.push_front((next_cave.clone(), new_counter));
                    }
                }
                _ => {}
            }
        }
    }
    path_count
}
