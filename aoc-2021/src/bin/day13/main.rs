use core::fmt;
use std::cmp;

use itertools::Itertools;

#[derive(Clone)]
struct Paper {
    points: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}
fn get_dimensions_from_points(points: &Vec<Point>) -> (usize, usize) {
    let mut width = 0;
    let mut height = 0;
    for point in points {
        let Point { x, y } = *point;
        if x > width {
            width = x;
        }
        if y > height {
            height = y;
        }
    }
    (width + 1, height + 1)
}

impl TryFrom<&Vec<Point>> for Paper {
    type Error = String;

    fn try_from(points: &Vec<Point>) -> Result<Self, Self::Error> {
        let (width, height) = get_dimensions_from_points(&points);
        let mut paper = Paper {
            points: vec![vec![false; width]; height],
            width,
            height,
        };
        for point in points {
            paper.points[point.y][point.x] = true;
        }
        Ok(paper)
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = vec![];
        for y in 0..self.height {
            let mut line = vec![];
            for x in 0..self.width {
                if self.points[y][x] {
                    line.push('#')
                } else {
                    line.push(' ')
                }
            }
            output.push(line.iter().collect::<String>())
        }
        write!(f, "{}", output.join("\n"))
    }
}

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
enum Fold {
    AlongX(usize),
    AlongY(usize),
}

impl TryFrom<&str> for Fold {
    type Error = String;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = input.split(" ").collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(format!("Unable to parse {} as Fold.", input));
        }
        let folds = parts[2].split("=").collect::<Vec<_>>();
        if folds.len() != 2 {
            return Err(format!("Unable to parse {} as Fold.", input));
        }
        match folds[0] {
            "x" => Ok(Self::AlongX(
                folds[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid x".to_string())?,
            )),
            "y" => Ok(Self::AlongY(
                folds[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid x".to_string())?,
            )),
            _ => Err("Invalid fold direction".to_string()),
        }
    }
}

fn parse(input: &str) -> (Vec<Point>, Vec<Fold>) {
    let mut points = vec![];
    let mut folds = vec![];

    let mut parsing_points = true;
    for line in input.lines() {
        if parsing_points && line.trim().len() == 0 {
            parsing_points = false;
            continue;
        }
        if parsing_points {
            points.push(line.try_into().unwrap());
        } else {
            folds.push(line.try_into().unwrap());
        }
    }

    (points, folds)
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(753, output_part_1);
    // assert_eq!("HZLEHJRK", output_part_2);
}

// fn fold_row(paper: &mut Paper, row: usize, x: usize) {
//     // shift left by x+1
//     let shift = x + 1;
//     let mut buffer = vec![];
//     let row = &mut paper.points[row];
//     let width = paper.width;
//     for index in 0..width {
//         if index < shift {
//             buffer.push(row[index]);
//         }
//         if index >= shift {
//             row[index - shift] = row[index];
//         }
//         if index >= width - shift {
//             row[index] = buffer[index - (width - shift)];
//         }
//     }

//     // fold
//     for index in 0..=x {
//         let source_index = width - x - 1 + index;
//         let target_index = (source_index + 2 * (x - index)) % width;
//         row[target_index] = row[target_index] || row[source_index];
//     }
// }
fn fold_row(paper: &mut Paper, row: usize, pivot: usize) {
    let row = &mut paper.points[row];
    let width = paper.width;
    let left_size = pivot - 1;

    let mut folded = vec![];
    for index in 1..=left_size {
        let left_value = row[pivot - index];
        let right_value = if pivot + index < width {
            row[pivot + index]
        } else {
            false
        };
        folded.push(left_value || right_value);
    }

    let mut target_index = 0;
    for folded_index in (0..folded.len()).rev() {
        row[target_index] = folded[folded_index];
        target_index += 1;
    }
}

fn fold_column(paper: &mut Paper, col: usize, pivot: usize) {
    let bottom_height = paper.height - pivot - 1;

    let mut folded = vec![];
    for index in 1..=bottom_height {
        let bottom_val = paper.points[pivot + index][col];
        let top_val = if index <= pivot {
            paper.points[pivot - index][col]
        } else {
            false
        };
        folded.push(bottom_val || top_val);
    }

    let mut target_index = 0;
    for folded_index in (0..folded.len()).rev() {
        paper.points[target_index][col] = folded[folded_index];
        target_index += 1;
    }
}

fn fold_paper(paper: &mut Paper, fold: &Fold) {
    match fold {
        &Fold::AlongX(pivot) => {
            let left_width = pivot;
            let right_width = paper.width - pivot - 1;
            for row in 0..paper.height {
                fold_row(paper, row, pivot)
            }
            paper.width = cmp::max(left_width, right_width);
        }
        &Fold::AlongY(pivot) => {
            let top_height = pivot;
            let bottom_height = paper.height - pivot - 1;
            for col in 0..paper.width {
                fold_column(paper, col, pivot)
            }
            paper.height = cmp::max(top_height, bottom_height);
        }
    };
}

fn count_visible(paper: &Paper) -> usize {
    let mut count = 0;
    for x in 0..paper.width {
        for y in 0..paper.height {
            if paper.points[y][x] {
                count += 1;
            }
        }
    }
    count
}

fn part_1(input: &str) -> usize {
    // let input = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5";
    // let input =
    //     "0,0\n2,0\n3,0\n6,0\n9,0\n0,1\n4,1\n6,2\n10,2\n0,3\n4,3\n1,4\n3,4\n6,4\n8,4\n9,4\n10,4";
    // let input = "0,0\n0,1\n0,4\n0,7";
    let (points, folds) = parse(input);
    let mut paper = Paper::try_from(&points).unwrap();

    fold_paper(&mut paper, &folds[0]);
    count_visible(&paper)
}

fn part_2(input: &str) -> () {
    let (points, folds) = parse(input);
    let mut paper = Paper::try_from(&points).unwrap();

    for fold in folds {
        fold_paper(&mut paper, &fold);
    }
    println!("{}", paper);
}
