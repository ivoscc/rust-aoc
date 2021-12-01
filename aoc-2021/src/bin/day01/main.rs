fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1462, output_part_1);
    assert_eq!(1497, output_part_2);
}

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("Can't parse depth"))
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|depths| depths[1] > depths[0])
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<usize>().expect("Can't parse depth"))
        .collect::<Vec<usize>>()
        .windows(3)
        .map(|depths| depths.iter().sum::<usize>())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|sums| sums[1] > sums[0])
        .count()
}
