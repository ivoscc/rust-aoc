use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Grid = [[bool; 1000]; 1000];

enum Operation {
    On,
    Off,
    Toggle
}

fn count_lights(grid: &Grid) -> usize {
    let mut counter = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] {
                counter += 1;
            }
        }
    }
    counter
}

fn update_grid_portion(grid: &mut Grid, operation: Operation, start: Point, end: Point) -> () {
    for x in start.x..=end.x {
        for y in start.y..=end.y {
            grid[y][x] = match operation {
                Operation::On => true,
                Operation::Off => false,
                Operation::Toggle => !grid[y][x],
            }
        }
    }

}

fn parse_line(line: &str) -> (Operation, Point, Point) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(.*) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let operation: Operation = match &captures[1] {
        "turn off" => Operation::Off,
        "turn on" => Operation::On,
        "toggle" => Operation::Toggle,
        _ => panic!("Unknown operation")
    };
    let start = Point { x: captures[2].parse().unwrap(), y: captures[3].parse().unwrap() };
    let end = Point{ x: captures[4].parse().unwrap(), y: captures[5].parse().unwrap() };
    (operation, start, end)
}

fn part_1(string: &str) -> usize {
    let mut grid: Grid = [[false; 1000]; 1000];
    string.lines()
          .for_each(|line| {
              let (operation, start, end) = parse_line(line);
              update_grid_portion(&mut grid, operation, start, end);
          });
    count_lights(&grid)
}

type DimmerGrid = [[usize; 1000]; 1000];

fn get_total_brightness(grid: &DimmerGrid) -> usize {
    let mut total_brightness = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            total_brightness += grid[y][x];
        }
    }
    total_brightness
}

fn part_2(string: &str) -> usize {
    let mut grid: DimmerGrid = [[0; 1000]; 1000];
    string.lines()
          .for_each(|line| {
              let (operation, start, end) = parse_line(line);
              for x in start.x..=end.x {
                  for y in start.y..=end.y {
                      grid[y][x] = match operation {
                          Operation::On => grid[y][x] + 1,
                          Operation::Off => if grid[y][x] == 0 { 0 } else {grid[y][x] - 1},
                          Operation::Toggle => grid[y][x] + 2,
                      }
                  }
              }
          });
    get_total_brightness(&grid)
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(543903, output_part_1);
    assert_eq!(14687245, output_part_2);
}
