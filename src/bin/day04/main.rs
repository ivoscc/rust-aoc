use crypto::{digest::Digest, md5::Md5};

fn part_1(input: &str) -> usize {
    let mut hasher = Md5::new();
    let secret = input.trim().to_string().to_owned();
    let mut output: usize = 1;
    loop {
        let test = secret.clone() + &output.to_string();
        hasher.input_str(&test);
        if hasher.result_str().starts_with("00000") {
            return output;
        }
        output += 1;
        if output % 100000 == 0 {
            println!("Trying {}...", output);
        }
        hasher.reset();
    }
}

fn part_2(input: &str) -> usize {
    let mut hasher = Md5::new();
    let secret = input.trim().to_string().to_owned();
    let mut output = 1;
    loop {
        let test = secret.clone() + &output.to_string();
        hasher.input_str(&test);
        if hasher.result_str().starts_with("000000") {
            return output;
        }
        output += 1;
        if output % 100000 == 0 {
            println!("Trying {}...", output);
        }
        hasher.reset();
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(282749, output_part_1);
    assert_eq!(9962624, output_part_2);
}
