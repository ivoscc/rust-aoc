use std::collections::{HashMap, HashSet};

type CharCounter = HashMap<char, usize>;
type Cache = HashMap<((char, char), usize), CharCounter>;

fn parse_input(input: &str) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut line_iterator = input.lines();
    let template = line_iterator.next().unwrap().chars().collect();
    let transformations = line_iterator
        .skip(1)
        .map(|line| {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            let mut source = parts[0].chars();
            let target = parts[1].chars().next().unwrap();
            ((source.next().unwrap(), source.next().unwrap()), target)
        })
        .collect();
    (template, transformations)
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(3306, output_part_1);
    assert_eq!(3760312702877, output_part_2);
}

fn part_1(input: &str) -> usize {
    let (mut template, transformations) = parse_input(input);
    for _ in 0..10 {
        let mut new_template = vec![template[0]];
        // Iterate wach window and either add the second part, or the newly
        // synthesized element and then the second part.
        template.windows(2).for_each(|chunk| {
            if let Some(target) = transformations.get(&(chunk[0], chunk[1])) {
                new_template.extend([*target, chunk[1]]);
            } else {
                new_template.extend([chunk[1]]);
            };
        });
        template = new_template;
    }

    let element_counter = template.iter().fold(HashMap::new(), |mut map, element| {
        *map.entry(element).or_insert(0) += 1;
        map
    });

    element_counter.values().max().unwrap() - element_counter.values().min().unwrap()
}

fn merge_counters(counter_a: CharCounter, counter_b: CharCounter) -> CharCounter {
    let unique_keys_in_both_counters =
        HashSet::<&char>::from_iter(counter_a.keys().chain(counter_b.keys()));
    return HashMap::from_iter(unique_keys_in_both_counters.iter().map(|&&character| {
        (
            character,
            counter_a.get(&character).unwrap_or(&0) + counter_b.get(&character).unwrap_or(&0),
        )
    }));
}

fn traverse(
    pair: (char, char),
    level: usize,
    target_level: usize,
    transformations: &HashMap<(char, char), char>,
    cache: &mut Cache,
) -> CharCounter {
    if let Some(cached) = cache.get(&(pair, level)) {
        return cached.clone();
    }
    let inserted = transformations.get(&pair);
    if level == target_level || inserted.is_none() {
        return HashMap::from_iter([(pair.1, 1)]);
    }

    let left_pair = (pair.0, *inserted.unwrap());
    let left_counter = traverse(left_pair, level + 1, target_level, transformations, cache);

    let right_pair = (*inserted.unwrap(), pair.1);
    let right_counter = traverse(right_pair, level + 1, target_level, transformations, cache);

    let counter = merge_counters(left_counter, right_counter);
    cache.insert((pair, level), counter.clone());

    return counter;
}

fn part_2(input: &str) -> usize {
    let target_level = 40;
    let (template, transformations) = parse_input(input);
    let mut cache = HashMap::new();
    let mut counter = HashMap::new();
    for pair in template.windows(2) {
        let pair_counter = traverse(
            (pair[0], pair[1]),
            0,
            target_level,
            &transformations,
            &mut cache,
        );
        counter = merge_counters(counter, pair_counter);
    }

    counter.values().max().unwrap() - counter.values().min().unwrap()
}
