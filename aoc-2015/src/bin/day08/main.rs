use regex::{Captures, Regex};

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1371, output_part_1);
    assert_eq!(2117, output_part_2);
}

fn part_2(input: &str) -> usize {
    let re =
        Regex::new(r#"(?P<hex>\\x[a-f0-9]{2}){1}|(?P<back>\\\\){1}|(?P<dquote>\\"){1}"#).unwrap();
    let escaped_count: usize = input
        .lines()
        .map(|line| {
            let escaped = re.replace_all(line, |caps: &Captures| {
                if let Some(hex) = &caps.name("hex") {
                    return format!(r#"\{}"#, hex.as_str());
                };

                if let Some(back) = &caps.name("back") {
                    return format!(r#"\\{}"#, back.as_str());
                };

                if let Some(dquote) = &caps.name("dquote") {
                    return format!(r#"\\{}"#, dquote.as_str());
                };

                String::from("")
            });
            (escaped.len() + 4) - line.len()
        })
        .sum();
    escaped_count
}

fn part_1(input: &str) -> usize {
    let re = Regex::new(r#"(\\x[a-f0-9]{2}){1}|(\\\\){1}|(\\"){1}"#).unwrap();
    input
        .lines()
        .map(|line| {
            let character_count = line.len();
            let in_memory_character_count = re.replace_all(line, String::from("_")).len();
            character_count - (in_memory_character_count - 2)
        })
        .sum()
}
