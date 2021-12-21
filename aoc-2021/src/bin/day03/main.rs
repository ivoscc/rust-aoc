fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1997414, output_part_1);
    assert_eq!(1032597, output_part_2);
}

fn get_most_common_bits(lines: &Vec<&str>) -> Vec<char> {
    let (one_counts, total_lines) = lines.iter().fold(
        (vec![], 0),
        |(mut one_counts, total): (Vec<usize>, usize), line: &&str| {
            line.char_indices().for_each(|(index, character)| {
                if index >= one_counts.len() {
                    one_counts.push(0);
                }
                if character == '1' {
                    one_counts[index] += 1;
                }
            });
            (one_counts, total + 1)
        },
    );
    let half = if total_lines % 2 == 0 {
        total_lines / 2
    } else {
        (total_lines / 2) + 1
    };
    one_counts
        .iter()
        .map(|ones| if *ones >= half { '1' } else { '0' })
        .collect()
}

fn part_1(input: &str) -> usize {
    let numbers = input.lines().collect::<Vec<&str>>();
    let binary_counts = get_most_common_bits(&numbers).iter().collect::<String>();
    let reverse_binary_counts = binary_counts
        .chars()
        .map(|x| if x == '1' { '0' } else { '1' })
        .collect::<String>();
    let gamma_rate = usize::from_str_radix(&binary_counts, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&reverse_binary_counts, 2).unwrap();
    epsilon_rate * gamma_rate
}

fn part_2(input: &str) -> usize {
    let mut numbers = input.lines().collect::<Vec<&str>>();
    let mut oxygen_rating = 0;
    for index in 0..numbers[0].len() {
        let most_common_bits = get_most_common_bits(&numbers);
        let most_common_bit_at_index = most_common_bits[index];
        let new_numbers = numbers
            .iter()
            .filter(|number| number.chars().nth(index).unwrap() == most_common_bit_at_index)
            .cloned()
            .collect();
        numbers = new_numbers;
        if numbers.len() == 1 {
            oxygen_rating = usize::from_str_radix(numbers[0], 2).unwrap();
            break;
        }
    }

    let mut numbers = input.lines().collect::<Vec<&str>>();
    let mut co_scrubber_rating = 0;
    for index in 0..numbers[0].len() {
        let most_common_bit_at_index = get_most_common_bits(&numbers)[index];
        let least_common_bit_at_index = if most_common_bit_at_index == '1' {
            '0'
        } else {
            '1'
        };
        numbers = numbers
            .iter()
            .filter(|number| number.chars().nth(index).unwrap() == least_common_bit_at_index)
            .cloned()
            .collect();
        if numbers.len() == 1 {
            co_scrubber_rating = usize::from_str_radix(numbers[0], 2).unwrap();
            break;
        }
    }

    oxygen_rating * co_scrubber_rating
}
