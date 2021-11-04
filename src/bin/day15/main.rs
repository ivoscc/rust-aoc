use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

type IngredientsMap = HashMap<String, Ingredient>;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(13882464, output_part_1);
    assert_eq!(11171160, output_part_2);
}

fn parse_ingredients(input: &str) -> IngredientsMap {
    let mut map: IngredientsMap = HashMap::new();
    for line in input.lines() {
        let line_parts: Vec<&str> = line.split(" ").collect();
        let ingredient = Ingredient {
            capacity: line_parts[2]
                .strip_suffix(",")
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            durability: line_parts[4]
                .strip_suffix(",")
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            flavor: line_parts[6]
                .strip_suffix(",")
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            texture: line_parts[8]
                .strip_suffix(",")
                .unwrap()
                .parse::<i64>()
                .unwrap(),
            calories: line_parts[10].parse::<i64>().unwrap(),
        };
        map.insert(
            line_parts[0].strip_suffix(":").unwrap().to_string(),
            ingredient,
        );
    }
    map
}

fn calculate_score(combination: &Vec<&String>, ingredients_map: &IngredientsMap) -> (usize, usize) {
    let mut counter: HashMap<String, usize> = HashMap::new();
    for ingredient_name in combination {
        let entry = counter.entry(ingredient_name.to_string()).or_insert(0);
        *entry += 1
    }
    let mut capacity = 0;
    let mut durability = 0;
    let mut flavor = 0;
    let mut texture = 0;
    let mut calories = 0;
    for (ingredient_name, ingredient) in ingredients_map {
        let ingredient_count: usize = *counter.get(ingredient_name).unwrap_or(&0);
        capacity += (ingredient_count as i64) * ingredient.capacity;
        durability += (ingredient_count as i64) * ingredient.durability;
        flavor += (ingredient_count as i64) * ingredient.flavor;
        texture += (ingredient_count as i64) * ingredient.texture;
        calories += (ingredient_count as i64) * ingredient.calories;
    }
    if capacity <= 0 || durability <= 0 || flavor <= 0 || texture <= 0 {
        return (0, calories as usize);
    }
    (
        (capacity * durability * flavor * texture) as usize,
        calories as usize,
    )
}

fn part_1(input: &str) -> usize {
    let ingredients = parse_ingredients(input);
    let mut highest_score = 0;
    for combination in ingredients.keys().combinations_with_replacement(100) {
        let (score, _) = calculate_score(&combination, &ingredients);
        if score > highest_score {
            highest_score = score;
        }
    }
    highest_score
}

fn calculate_score_with_calories(
    combination: &Vec<&String>,
    ingredients_map: &IngredientsMap,
    target_calories: usize,
) -> usize {
    let (score, calories) = calculate_score(combination, ingredients_map);
    if calories != target_calories {
        0
    } else {
        score
    }
}

fn part_2(input: &str) -> usize {
    let ingredients = parse_ingredients(input);
    let mut highest_score = 0;
    for combination in ingredients.keys().combinations_with_replacement(100) {
        let score = calculate_score_with_calories(&combination, &ingredients, 500);
        if score > highest_score {
            highest_score = score;
        }
    }
    highest_score
}
