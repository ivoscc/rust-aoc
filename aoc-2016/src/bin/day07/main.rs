use std::{error::Error, ops::RangeBounds};

use fancy_regex::Regex;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input)?;
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input)?;
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(115, output_part_1);
    assert_eq!(231, output_part_2);
    Ok(())
}

fn part_1(input: &str) -> Result<usize, Box<dyn Error>> {
    let abba_regex = Regex::new(r"(?:([a-z])(((?!\1)(?:[a-z])))\2\1)").unwrap();
    let hypernet_regex = Regex::new(r"\[(.*?)\]").unwrap();
    let mut support_tls = 0;
    for line in input.lines() {
        if !abba_regex.is_match(line)? {
            continue;
        }
        let mut hypernet_has_abba = false;
        for hypernet in hypernet_regex.find_iter(line) {
            if abba_regex.is_match(hypernet?.as_str())? {
                hypernet_has_abba = true;
            }
        }
        if !hypernet_has_abba {
            support_tls += 1;
        }
    }
    Ok(support_tls)
}

fn part_2(input: &str) -> Result<usize, Box<dyn Error>> {
    let hypernet_regex = Regex::new(r"\[(.*?)\]").unwrap();
    let mut support_tls = 0;
    for line in input.lines() {
        let matched_abas = line.chars().collect::<Vec<char>>();
        let matched_abas = matched_abas
            .windows(3)
            .fold((false, vec![]), |(in_hypernet, mut output), triplet| {
                if in_hypernet {
                    if triplet[0] == ']' || triplet[1] == ']' || triplet[2] == ']' {
                        return (false, output);
                    } else {
                        return (in_hypernet, output);
                    }
                } else {
                    if triplet[0] == '[' || triplet[1] == '[' || triplet[2] == '[' {
                        return (true, output);
                    }
                    if triplet[0] == triplet[2]
                        && triplet[0] != triplet[1]
                        && triplet[0] != ']'
                        && triplet[0] != '['
                        && triplet[1] != ']'
                        && triplet[1] != ']'
                        && triplet[2] != '['
                        && triplet[2] != '['
                    {
                        output.push(triplet.iter().clone().collect::<String>())
                    }
                    return (in_hypernet, output);
                }
            })
            .1;
        let hypernets = hypernet_regex
            .find_iter(line)
            .map(|m| m.expect("Failed regex execution").as_str())
            .collect::<Vec<&str>>();
        let mut contains = false;
        for aba in matched_abas {
            let characters = aba.chars().collect::<Vec<char>>();
            let bab: String = [characters[1], characters[0], characters[1]]
                .iter()
                .collect();
            for hypernet in &hypernets {
                if hypernet.contains(&bab) {
                    support_tls += 1;
                    contains = true;
                    break;
                }
            }
            if contains {
                break;
            }
        }
    }
    Ok(support_tls)
}
