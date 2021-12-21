use std::cmp;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl TryFrom<&str> for Point {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = input.split(",").collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(format!("Unable to parse {} as point.", input));
        }
        Ok(Self {
            x: parts[0].parse().map_err(|_| "Invalid x".to_string())?,
            y: parts[1].parse().map_err(|_| "Invalid y".to_string())?,
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl TryFrom<&str> for Line {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = input.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!("Unable to parse {} as line.", input));
        }
        Ok(Self {
            start: parts[0].try_into()?,
            end: parts[2].try_into()?,
        })
    }
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.x != self.end.x && self.start.y != self.end.y
    }
}

fn mark_line(sea_map: &mut Vec<Vec<usize>>, line: &Line) {
    let Line { start, end } = line;

    let x_step = match (start.x, end.x) {
        (start, end) if start < end => 1,
        (start, end) if start > end => -1,
        _ => 0,
    };
    let y_step = match (start.y, end.y) {
        (start, end) if start < end => 1,
        (start, end) if start > end => -1,
        _ => 0,
    };

    let min_x = cmp::min(start.x, end.x) as isize;
    let max_x = cmp::max(start.x, end.x) as isize;
    let min_y = cmp::min(start.y, end.y) as isize;
    let max_y = cmp::max(start.y, end.y) as isize;

    let mut x = start.x as isize;
    let mut y = start.y as isize;
    while x <= max_x && y <= max_y && x >= min_x && y >= min_y {
        sea_map[y as usize][x as usize] += 1;
        x += x_step;
        y += y_step;
    }
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(5442, output_part_1);
    assert_eq!(19571, output_part_2);
}

fn part_1(input: &str) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;

    let lines: Vec<Line> = input
        .lines()
        .map(|l| l.try_into().unwrap())
        .filter(|line: &Line| !line.is_diagonal())
        .inspect(|line| {
            let biggest_x = cmp::max(line.start.x, line.end.x);
            let biggest_y = cmp::max(line.start.y, line.end.y);
            if biggest_x > max_x {
                max_x = biggest_x;
            }
            if biggest_y > max_y {
                max_y = biggest_y;
            }
        })
        .collect();

    let mut sea_map = vec![vec![0; max_x + 1]; max_y + 1];
    for line in lines {
        mark_line(&mut sea_map, &line)
    }
    sea_map
        .iter()
        .map(|line| line.iter().filter(|val| **val > 1).count())
        .sum()
}

fn part_2(input: &str) -> usize {
    let mut max_x = 0;
    let mut max_y = 0;

    let lines: Vec<Line> = input
        .lines()
        .map(|l| l.try_into().unwrap())
        .inspect(|line: &Line| {
            let biggest_x = cmp::max(line.start.x, line.end.x);
            let biggest_y = cmp::max(line.start.y, line.end.y);
            if biggest_x > max_x {
                max_x = biggest_x;
            }
            if biggest_y > max_y {
                max_y = biggest_y;
            }
        })
        .collect();

    let mut sea_map = vec![vec![0; max_x + 1]; max_y + 1];
    for line in lines {
        mark_line(&mut sea_map, &line);
    }
    sea_map
        .iter()
        .map(|line| line.iter().filter(|val| **val > 1).count())
        .sum()
}
