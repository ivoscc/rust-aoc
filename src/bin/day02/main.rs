fn parse_line(line: &str) -> (u32, u32, u32) {
    let side_sizes: Vec<u32> = line.split("x").map(|f| f.parse().unwrap()).collect();
    (side_sizes[0], side_sizes[1], side_sizes[2])
}

fn part_1(input: &str) -> u32 {
    input.lines()
        .map(parse_line)
        .map(|(length, width, height)| {
            let areas = [length * width, width * height, height * length];
            let smallest_side = areas.iter().min().unwrap();
            let total: u32 = areas.iter().map(|side| side * 2).sum();
            total + smallest_side
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    input.lines()
        .map(parse_line)
        .map(|(length, width, height)| {
            let mut sides = [length, width, height];
            sides.sort();
            let ribbon_for_wrap = sides[0] * 2 + sides[1] * 2;
            let ribbon_for_bow = length * width * height;
            ribbon_for_bow + ribbon_for_wrap
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    let output_part_2 = part_2(input);
    println!("Part 1 output is {:?}", output_part_1);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1598415, output_part_1);
    assert_eq!(3812909, output_part_2);
}
