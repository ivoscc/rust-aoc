fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(391671, output_part_1);
    // assert_eq!(_, output_part_2);
}

fn transform_school(fishes: &mut Vec<usize>) -> () {
    let mut new_fishes = 0;
    for fish in fishes.iter_mut() {
        match fish {
            0 => {
                new_fishes += 1;
                *fish = 6;
            }
            _ => *fish -= 1,
        }
    }
    fishes.extend(vec![8; new_fishes]);
}

fn part_1(input: &str) -> usize {
    let mut fishes = input
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    for _ in 0..80 {
        transform_school(&mut fishes);
    }
    fishes.len()
}

fn part_2(input: &str) -> usize {
    let mut timer_counts = [0; 9];
    input
        .split(",")
        .map(|n| n.trim().parse::<usize>().unwrap())
        .for_each(|n| timer_counts[n] += 1);

    for _ in 0..256 {
        let ready_to_procreate = timer_counts[0];
        timer_counts.rotate_left(1);
        timer_counts[6] += ready_to_procreate;
    }
    timer_counts.iter().sum()
}
