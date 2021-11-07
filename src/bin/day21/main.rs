use itertools::Itertools;
use std::{cmp, collections::HashMap};

#[derive(Debug, Clone)]
struct Stats {
    hit_points: i64,
    damage: i64,
    armor: i64,
}

#[derive(Debug, Clone)]
struct Item {
    name: String,
    cost: i64,
    damage_increment: i64,
    armor_increment: i64,
}

#[derive(Debug)]
struct Store {
    weapons: HashMap<String, Item>,
    armor: HashMap<String, Item>,
    rings: HashMap<String, Item>,
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(78, output_part_1);
    assert_eq!(148, output_part_2);
}

fn get_available_weapons() -> Store {
    let input = include_str!("store.txt");
    let mut weapons: HashMap<String, Item> = HashMap::new();
    let mut armor: HashMap<String, Item> = HashMap::new();
    let mut rings: HashMap<String, Item> = HashMap::new();
    let mut current_section = String::from("");
    for line in input.lines() {
        let line_parts: Vec<String> = line
            .split(" ")
            .map(|line_parts| line_parts.trim().to_string())
            .filter(|part| part != "")
            .collect();
        if line_parts.len() == 0 {
            continue;
        }
        if line_parts[0].ends_with(":") {
            current_section = line_parts[0].strip_suffix(":").unwrap().to_string();
            continue;
        }
        match current_section.as_str() {
            "Weapons" => weapons.insert(
                line_parts[0].to_string(),
                Item {
                    name: line_parts[0].to_string(),
                    cost: line_parts[1].parse::<i64>().unwrap(),
                    damage_increment: line_parts[2].parse::<i64>().unwrap(),
                    armor_increment: line_parts[3].parse::<i64>().unwrap(),
                },
            ),
            "Armor" => armor.insert(
                line_parts[0].to_string(),
                Item {
                    name: line_parts[0].to_string(),
                    cost: line_parts[1].parse::<i64>().unwrap(),
                    damage_increment: line_parts[2].parse::<i64>().unwrap(),
                    armor_increment: line_parts[3].parse::<i64>().unwrap(),
                },
            ),
            "Rings" => rings.insert(
                format!("{} {}", line_parts[0], line_parts[1]),
                Item {
                    name: format!("{} {}", line_parts[0], line_parts[1]),
                    cost: line_parts[2].parse::<i64>().unwrap(),
                    damage_increment: line_parts[3].parse::<i64>().unwrap(),
                    armor_increment: line_parts[4].parse::<i64>().unwrap(),
                },
            ),
            _ => panic!("Unknown store section."),
        };
    }
    Store {
        weapons,
        armor,
        rings,
    }
}

fn parse_boss_stats(input: &str) -> Stats {
    let parts: Vec<i64> = input
        .lines()
        .map(|line| line.split(" ").last().unwrap().parse::<i64>().unwrap())
        .collect();
    Stats {
        hit_points: parts[0],
        damage: parts[1],
        armor: parts[2],
    }
}

fn player_wins_fight(player: &Stats, boss: &Stats) -> bool {
    let mut player = player.clone();
    let mut boss = boss.clone();
    let mut turn = 0;
    loop {
        if turn % 2 == 0 {
            let damage: i64 = cmp::max(player.damage - boss.armor, 1);
            boss.hit_points -= damage;
        } else {
            let damage: i64 = cmp::max(boss.damage - player.armor, 1);
            player.hit_points -= damage;
        }
        if boss.hit_points <= 0 {
            return true;
        }
        if player.hit_points <= 0 {
            return false;
        }
        turn += 1;
    }
}

fn wear_items(player: &Stats, items: &Vec<&Item>) -> Stats {
    let mut updated_player = Stats { ..*player };
    for item in items {
        updated_player.armor += item.armor_increment;
        updated_player.damage += item.damage_increment;
    }
    updated_player
}

fn get_all_item_combinations(store: &Store) -> Vec<Vec<&Item>> {
    let available_weapons = store
        .weapons
        .values()
        .map(|weapon| Some(weapon))
        .collect_vec();
    let available_armor = store
        .clone()
        .armor
        .values()
        .clone()
        .map(|armor| Some(armor))
        .chain([None].into_iter())
        .collect_vec();
    let available_rings = store
        .rings
        .values()
        .map(|ring| Some(vec![ring]))
        .chain(
            store
                .rings
                .values()
                .combinations(2)
                .map(|rings| Some(vec![rings[0], rings[1]])),
        )
        .chain([None].into_iter())
        .collect_vec();
    let mut output = vec![];
    for weapon in &available_weapons {
        for armor in &available_armor {
            for rings in &available_rings {
                let mut items = vec![];
                if let Some(weapon) = weapon {
                    items.push(weapon.clone());
                }
                if let Some(armor) = armor {
                    items.push(armor.clone());
                }
                if let Some(rings) = rings {
                    for ring in rings {
                        items.push(ring.clone());
                    }
                }
                output.push(items);
            }
        }
    }
    output
}

fn part_1(input: &str) -> usize {
    let player = Stats {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };
    let boss = parse_boss_stats(input);
    let store = get_available_weapons();
    let mut lowest_cost_for_win = 0;
    for item_set in get_all_item_combinations(&store) {
        if player_wins_fight(&wear_items(&player, &item_set), &boss) {
            let cost = item_set.iter().map(|item| item.cost as usize).sum();
            if lowest_cost_for_win == 0 || cost < lowest_cost_for_win {
                lowest_cost_for_win = cost;
            }
        }
    }
    lowest_cost_for_win
}

fn part_2(input: &str) -> usize {
    let player = Stats {
        hit_points: 100,
        damage: 0,
        armor: 0,
    };
    let boss = parse_boss_stats(input);
    let store = get_available_weapons();
    let mut highest_cost_for_losing = 0;
    for item_set in get_all_item_combinations(&store) {
        if !(player_wins_fight(&wear_items(&player, &item_set), &boss)) {
            let cost = item_set.iter().map(|item| item.cost as usize).sum();
            if cost > highest_cost_for_losing {
                highest_cost_for_losing = cost;
            }
        }
    }
    highest_cost_for_losing
}
