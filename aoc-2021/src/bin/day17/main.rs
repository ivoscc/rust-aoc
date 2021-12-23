use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(12246, output_part_1);
    assert_eq!(3528, output_part_2);
}

#[derive(Debug)]
struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[derive(Debug)]
struct Probe {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
}

impl Probe {
    fn step(&mut self) {
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
        self.x_vel -= match self.x_vel {
            x_vel if x_vel < 0 => -1,
            x_vel if x_vel > 0 => 1,
            _ => 0,
        };
        self.y_vel -= 1;
    }
}

fn parse_input(input: &str) -> TargetArea {
    let re = Regex::new(
        r"x=(?P<x_min>-?[0-9]+)\.\.(?P<x_max>-?[0-9]+).*?y=(?P<y_min>-?[0-9]+)\.\.(?P<y_max>-?[0-9]+)",
    )
    .unwrap();
    let caps = re.captures(input).unwrap();
    TargetArea {
        x_min: caps["x_min"].parse().unwrap(),
        x_max: caps["x_max"].parse().unwrap(),
        y_min: caps["y_min"].parse().unwrap(),
        y_max: caps["y_max"].parse().unwrap(),
    }
}

fn area_contains_probe(target_area: &TargetArea, probe: &Probe) -> bool {
    let TargetArea {
        x_max,
        x_min,
        y_max,
        y_min,
    } = target_area;
    let Probe { x_pos, y_pos, .. } = probe;
    !(x_pos < x_min || x_pos > x_max || y_pos < y_min || y_pos > y_max)
}

fn probe_overshot(target_area: &TargetArea, probe: &Probe) -> bool {
    let TargetArea { x_max, y_min, .. } = target_area;
    let Probe { x_pos, y_pos, .. } = probe;
    y_pos < y_min || x_pos > x_max
}

fn simulate(x_vel: i32, y_vel: i32, target_area: &TargetArea) -> Option<i32> {
    let mut probe = Probe {
        x_pos: 0,
        y_pos: 0,
        x_vel,
        y_vel,
    };
    let mut highest = 0;
    loop {
        probe.step();
        if probe.y_pos > highest {
            highest = probe.y_pos;
        }
        if area_contains_probe(target_area, &probe) {
            break;
        }
        if probe_overshot(target_area, &probe) {
            return None;
        }
    }

    Some(highest)
}

fn part_1(input: &str) -> usize {
    let target_area = parse_input(input);
    let mut total_highest_y = 0;
    for x_vel in 1..=target_area.x_max {
        for y_vel in 1..=(target_area.y_min).abs() {
            if let Some(highest_y) = simulate(x_vel, y_vel, &target_area) {
                if highest_y > total_highest_y {
                    total_highest_y = highest_y;
                }
            }
        }
    }
    total_highest_y as usize
}

fn part_2(input: &str) -> usize {
    let target_area = parse_input(input);
    let mut valid_initial_velocity_counter = 0;
    for x_vel in 1..=target_area.x_max {
        for y_vel in -(target_area.y_min.abs())..=target_area.y_min.abs() {
            if simulate(x_vel, y_vel, &target_area).is_some() {
                valid_initial_velocity_counter += 1;
            }
        }
    }
    valid_initial_velocity_counter
}
