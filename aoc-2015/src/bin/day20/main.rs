fn main() {
    let input = include_str!("input.txt");
    // let output_part_1 = part_1(input);
    let output_part_1 = part_1_fast(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(831600, output_part_1);
    assert_eq!(884520, output_part_2);
}

fn part_1_fast(input: &str) -> usize {
    let target_presents = input.trim().parse::<usize>().unwrap();
    let vector_size = target_presents / 10 + 1;
    let mut values: Vec<usize> = vec![0; vector_size];
    for i in 1..vector_size {
        for j in (i..vector_size).step_by(i) {
            values[j] += i * 10;
        }
    }

    for (house_number, value) in values.iter().enumerate() {
        if *value >= target_presents {
            return house_number;
        }
    }
    0
}

fn part_1(input: &str) -> usize {
    let target_presents = input.trim().parse::<usize>().unwrap();
    let mut house_number = 1;
    let mut max_so_far = 0;
    loop {
        let presents_for_house = calculate_presents_for_house(house_number);
        if presents_for_house >= target_presents {
            return house_number;
        }

        if presents_for_house > max_so_far {
            max_so_far = presents_for_house;
        }
        if (house_number % 10000) == 0 {
            println!("House = {}; Max is {}", house_number, max_so_far);
        }
        house_number += 1
    }
}

fn calculate_presents_for_house(n: usize) -> usize {
    let mut total_presents = n + 1;
    let mut smallest = (n as f64).sqrt().floor() as usize;
    let mut current = 2;
    while current <= smallest {
        if n % current == 0 {
            let counterpart = n / current;
            if current != counterpart {
                total_presents += current + counterpart;
            } else {
                total_presents += current;
            }
            smallest = counterpart;
        }
        current += 1;
    }
    total_presents * 10
}

fn part_2(input: &str) -> usize {
    let target_presents = input.trim().parse::<usize>().unwrap();
    let vector_size = target_presents / 10 + 1;
    let mut values: Vec<usize> = vec![0; vector_size];
    for i in 1..vector_size {
        for j in (i..vector_size).step_by(i).take(50) {
            values[j] += i * 11;
        }
    }

    for (house_number, value) in values.iter().enumerate() {
        if *value >= target_presents {
            return house_number;
        }
    }
    0
}
