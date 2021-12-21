use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crypto::aesni::AesNiDecryptor;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(349, output_part_1);
    assert_eq!(1070957, output_part_2);
}

fn part_1(input: &str) -> usize {
    /*
     * Numbers represented with an unique number of segments
     * 1 -> 2 segments
     * 4 -> 4 segments
     * 7 -> 3 segments
     * 8 -> 7 segments
     * */
    let unique_lengths: HashSet<usize> = HashSet::from_iter([2, 3, 4, 7]);
    input
        .lines()
        .map(|line| {
            line.split("|")
                .last()
                .unwrap()
                .split(" ")
                .map(|part| part.trim())
                .filter(|part| part.len() > 0)
                .filter(|part| unique_lengths.contains(&part.len()))
                .count()
        })
        .sum()
}

fn decode(all_segments: &Vec<String>, output: &Vec<String>) -> usize {
    /*
     * | segment | number | how to know?                                  | order? |
     * |---------+--------+-----------------------------------------------+--------|
     * | ab      |      1 | length 2                                      |      1 |
     * | eafb    |      4 | length 4                                      |      2 |
     * | dab     |      7 | length 3                                      |      3 |
     * | acedgfb |      8 | length 7                                      |      4 |
     * | cdfbe   |      5 | length 5, has all in (4 - 1)                  |      5 |
     * | gcdfa   |      2 | length 5, has 2 segments after - 5            |      6 |
     * | fbcad   |      3 | length 5, has 1 segment  after - 5            |      7 |
     * | cdfgeb  |      6 | length 6, doesn't have both in 1              |      8 |
     * | cagedb  |      0 | length 6, has 3 segments after -4 [is not 6]  |      9 |
     * | cefabd  |      9 | length 6, has 2 segments after -4  [is not 6] |     10 |
     * */
    let mut decoding_map: HashMap<&str, usize> = HashMap::new();
    let mut encoding_map: HashMap<usize, HashSet<char>> = HashMap::new();

    for segment in all_segments {
        match segment.len() {
            2 => {
                decoding_map.insert(segment, 1);
                encoding_map.insert(1, segment.chars().collect::<HashSet<char>>());
            }
            3 => {
                decoding_map.insert(segment, 7);
                encoding_map.insert(7, segment.chars().collect::<HashSet<char>>());
            }
            4 => {
                decoding_map.insert(segment, 4);
                encoding_map.insert(4, segment.chars().collect::<HashSet<char>>());
            }
            7 => {
                decoding_map.insert(segment, 8);
                encoding_map.insert(8, segment.chars().collect::<HashSet<char>>());
            }
            _ => {}
        }
    }

    // get 5
    let encoded_5 = all_segments
        .iter()
        .filter(|segment| segment.len() == 5)
        .filter(|segment| {
            let segment_chars = segment.chars().collect::<HashSet<char>>();
            let four_minus_1 = encoding_map
                .get(&4)
                .unwrap()
                .difference(encoding_map.get(&1).unwrap());
            for character in four_minus_1 {
                if !segment_chars.contains(character) {
                    return false;
                }
            }
            true
        })
        .last()
        .unwrap();
    decoding_map.insert(&encoded_5, 5);
    encoding_map.insert(5, encoded_5.chars().collect::<HashSet<_>>());

    // get 2 and 3
    all_segments
        .iter()
        .filter(|segment| segment.len() == 5)
        .for_each(|segment| {
            if let Some(5) = decoding_map.get(segment.as_str()) {
                return;
            }
            let five = encoding_map.get(&5).unwrap();
            let remaining_number = segment
                .chars()
                .collect::<HashSet<_>>()
                .difference(five)
                .collect::<HashSet<_>>()
                .len();
            match remaining_number {
                2 => {
                    decoding_map.insert(&segment, 2);
                    encoding_map.insert(2, segment.chars().collect::<HashSet<_>>());
                }
                1 => {
                    decoding_map.insert(&segment, 3);
                    encoding_map.insert(3, segment.chars().collect::<HashSet<_>>());
                }
                _ => {}
            }
        });

    // get 6
    let encoded_6 = all_segments
        .iter()
        .filter(|segment| segment.len() == 6)
        .filter(|segment| {
            encoding_map
                .get(&1)
                .unwrap()
                .iter()
                .filter(|character| segment.contains(&character.to_string()))
                .count()
                < 2
        })
        .last()
        .unwrap();
    decoding_map.insert(&encoded_6, 6);
    encoding_map.insert(6, encoded_6.chars().collect::<HashSet<_>>());

    // get 0 and 9
    all_segments
        .iter()
        .filter(|segment| segment.len() == 6)
        .for_each(|segment| {
            if let Some(6) = decoding_map.get(segment.as_str()) {
                return;
            }
            let four = encoding_map.get(&4).unwrap();
            let remaining_number = segment
                .chars()
                .collect::<HashSet<_>>()
                .difference(four)
                .collect::<HashSet<_>>()
                .len();
            match remaining_number {
                2 => {
                    decoding_map.insert(&segment, 9);
                    encoding_map.insert(9, segment.chars().collect::<HashSet<_>>());
                }
                3 => {
                    decoding_map.insert(&segment, 0);
                    encoding_map.insert(0, segment.chars().collect::<HashSet<_>>());
                }
                _ => {}
            }
        });

    output.iter().fold(0, |result, segment| {
        let next_digit = decoding_map.get(&segment.as_str()).unwrap();
        result * 10 + next_digit
    })
}

fn part_2(input: &str) -> usize {
    // let input = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    // let input =
    //     "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    input
        .lines()
        .map(|line| {
            let parts = line
                .split(" ")
                .map(|part| part.trim())
                .filter(|part| part.len() > 0 && *part != "|")
                .map(|part| {
                    let mut chars = part.chars().collect::<Vec<_>>();
                    chars.sort();
                    chars.iter().collect::<String>()
                })
                .collect::<Vec<_>>();
            let all_segments = parts
                .iter()
                .take(10)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let output = parts
                .iter()
                .skip(10)
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            decode(&all_segments, &output)
        })
        .sum()
}
