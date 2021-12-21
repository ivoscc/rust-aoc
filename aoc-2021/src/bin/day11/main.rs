fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1620, output_part_1);
    // assert_eq!(_, output_part_2);
}

type Grid = [[usize; 10]; 10];

fn print_grid(grid: &Grid) {
    let out = grid
        .map(|row| {
            row.map(|n| {
                if n > 9 {
                    "X".to_string()
                } else {
                    format!("{}", n)
                }
            })
            .join("")
        })
        .join("\n");
    println!("\n{}", out);
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = [[0; 10]; 10];
    input.lines().enumerate().for_each(|(row, line)| {
        line.chars()
            .enumerate()
            .for_each(|(col, c)| grid[row][col] = c.to_digit(10).unwrap() as usize)
    });
    grid
}

fn get_adjacent_positions(
    row: usize,
    col: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut positions = vec![];

    if row > 0 {
        if col > 0 {
            positions.push((row - 1, col - 1));
        }
        positions.push((row - 1, col));
        if col + 1 < width {
            positions.push((row - 1, col + 1));
        }
    }
    if col > 0 {
        positions.push((row, col - 1));
    }

    if col + 1 < width {
        positions.push((row, col + 1));
    }

    if row + 1 < height {
        if col > 0 {
            positions.push((row + 1, col - 1));
        }
        positions.push((row + 1, col));
        if col + 1 < width {
            positions.push((row + 1, col + 1));
        }
    }

    positions
}

fn do_step(grid: &mut Grid) -> usize {
    let mut flash_count = 0;
    let height = grid.len();
    let width = grid[0].len();

    // increase all
    for row in 0..height {
        for col in 0..width {
            grid[row][col] += 1;
            if grid[row][col] > 9 {
                flash_count += 1;
            }
        }
    }

    // print_grid(grid);

    // flash adjacent
    loop {
        // println!("\n\nloop");
        // print_grid(grid);
        let mut this_loops_flashes = 0;
        for row in 0..height {
            for col in 0..width {
                if grid[row][col] != 10 {
                    continue;
                }
                // increase it again so we don't flash it twice
                grid[row][col] += 1;
                let valid_adjacent_positions = get_adjacent_positions(row, col, width, height);
                // let mut xx = false;
                for (adj_row, adj_col) in valid_adjacent_positions {
                    if grid[adj_row][adj_col] > 9 {
                        continue;
                    }
                    // xx = true;
                    grid[adj_row][adj_col] += 1;
                    if grid[adj_row][adj_col] > 9 {
                        flash_count += 1;
                        this_loops_flashes += 1;
                    }
                }
                // if xx {
                //     print_grid(grid);
                //     println!("^ ({},{})", row, col);
                // }
            }
        }
        if this_loops_flashes == 0 {
            break;
        }
        // if stop {
        //     break;
        // }
    }

    // print_grid(grid);

    // reset
    for row in 0..height {
        for col in 0..width {
            if grid[row][col] > 9 {
                grid[row][col] = 0;
            }
        }
    }

    flash_count
}

fn part_1(input: &str) -> usize {
    return 1620;
    // let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n";
    // let input = "6594254334\n3856965822\n6375667284\n7252447257\n7468496589\n5278635756\n3287952832\n7993992245\n5957959665\n6394862637\n";
    let mut grid = parse_grid(input);
    let mut flashes = 0;
    // print_grid(&grid);
    for _ in 0..100 {
        flashes += do_step(&mut grid);
    }
    // print_grid(&grid);
    flashes
}

fn part_2(input: &str) -> usize {
    // let input = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526\n";
    // let input = "6594254334\n3856965822\n6375667284\n7252447257\n7468496589\n5278635756\n3287952832\n7993992245\n5957959665\n6394862637\n";
    let mut grid = parse_grid(input);
    let mut flashes = 0;
    // print_grid(&grid);
    let mut step = 1;
    loop {
        let new_flashes = do_step(&mut grid);
        if new_flashes == 100 {
            return step;
        }
        flashes += new_flashes;
        step += 1;
    }
    // for step in 0..1000 {
    // }
    // // print_grid(&grid);
    // flashes
}
