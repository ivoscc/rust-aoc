fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(983, output_part_1);
    assert_eq!(1836, output_part_2);
}

type Triangle = [usize; 3];
fn parse_input(input: &str) -> Vec<Triangle> {
    input
        .lines()
        .map(|line| {
            let sides: Vec<usize> = line
                .split(" ")
                .filter_map(|part| {
                    if let Ok(item) = part.trim().parse::<usize>() {
                        Some(item)
                    } else {
                        None
                    }
                })
                .collect();
            assert!(
                sides.len() == 3,
                "Unable to parse the 3 sides of the triangle."
            );
            [sides[0], sides[1], sides[2]]
        })
        .collect()
}

fn count_possible_triangles(triangles: &Vec<Triangle>) -> usize {
    triangles
        .iter()
        .filter(|[a, b, c]| a + b > *c && b + c > *a && a + c > *b)
        .count()
}

fn shift_triangles(input: Vec<Triangle>) -> Vec<Triangle> {
    input
        .chunks(3)
        .flat_map(|triangles| {
            let t0 = triangles[0];
            let t1 = triangles[1];
            let t2 = triangles[2];
            [
                [t0[0], t1[0], t2[0]],
                [t0[1], t1[1], t2[1]],
                [t0[2], t1[2], t2[2]],
            ]
        })
        .collect()
}

fn part_1(input: &str) -> usize {
    let triangles = parse_input(input);
    count_possible_triangles(&triangles)
}

fn part_2(input: &str) -> usize {
    let triangles = shift_triangles(parse_input(input));
    count_possible_triangles(&triangles)
}
