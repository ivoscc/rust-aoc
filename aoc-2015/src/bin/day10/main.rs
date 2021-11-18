fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    // 246574
    // 82350
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(252594, output_part_1);
    assert_eq!(3579328, output_part_2);
}

fn mutate_sequence(sequence: &str) -> String {
    if sequence.len() == 0 {
        return String::from("");
    }
    let mut output: Vec<char> = vec![];
    let mut current_count = 1;
    let mut last_character = sequence.chars().next().unwrap();
    sequence.chars().skip(1).for_each(|character| {
        if last_character != character {
            output.push(char::from_digit(current_count, 10).unwrap());
            output.push(last_character);
            current_count = 0;
        }
        last_character = character;
        current_count += 1;
    });
    output.push(char::from_digit(current_count, 10).unwrap());
    output.push(last_character);
    output.into_iter().collect()
}

fn part_1(input: &str) -> usize {
    let mut sequence: String = String::from(input.trim());
    for _ in 0..40 {
        sequence = mutate_sequence(&sequence);
    }
    sequence.len()
}

fn part_2(input: &str) -> usize {
    let mut sequence: String = String::from(input.trim());
    for _ in 0..50 {
        sequence = mutate_sequence(&sequence);
    }
    sequence.len()
}
