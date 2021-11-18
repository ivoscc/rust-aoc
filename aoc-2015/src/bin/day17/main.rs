fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1638, output_part_1);
    assert_eq!(17, output_part_2);
}

fn parse_available_containers(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn count_available_ways(target: i64, available_containers: &[i64]) -> usize {
    if target == 0 {
        return 1;
    }
    if target < 0 || available_containers.is_empty() {
        return 0;
    }
    // use first
    let available_ways_using_first =
        count_available_ways(target - available_containers[0], &available_containers[1..]);
    // don't use first
    let available_ways_not_using_first = count_available_ways(target, &available_containers[1..]);
    available_ways_using_first + available_ways_not_using_first
}

fn get_available_ways(
    target: i64,
    available_containers: &[i64],
    current_path: &Vec<i64>,
    output: &mut Vec<usize>,
) -> () {
    if target == 0 {
        output.push(current_path.len());
        return;
    }
    if target < 0 || available_containers.is_empty() {
        return;
    }

    let mut current_path_with = current_path.clone();
    current_path_with.push(available_containers[0]);
    // use first
    get_available_ways(
        target - available_containers[0],
        &available_containers[1..],
        &current_path_with,
        output,
    );

    // don't use first
    get_available_ways(target, &available_containers[1..], &current_path, output);
}

fn part_1(input: &str) -> usize {
    let containers = parse_available_containers(input);
    count_available_ways(150, &containers)
}

fn part_2(input: &str) -> usize {
    let containers = parse_available_containers(input);
    let mut output = vec![];
    get_available_ways(150, &containers, &vec![], &mut output);

    // get min
    let min_containers = output.iter().min().unwrap();
    output
        .iter()
        .filter(|&count| count == min_containers)
        .count()
}
