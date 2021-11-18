const MAX_WIDTH: usize = 100;
const MAX_HEIGHT: usize = 100;

type Grid = [[bool; MAX_WIDTH]; MAX_HEIGHT];

fn get_grid_display(grid: &Grid) -> String {
    let mut output: Vec<String> = vec![];
    for row in grid {
        output.push(
            row.into_iter()
                .map(|&light| if light { '#' } else { '.' })
                .collect::<String>(),
        );
    }
    output.join("\n")
}

fn main() {
    let input = include_str!("input.txt");
    let output_part_1 = part_1(input);
    println!("Part 1 output is {:?}", output_part_1);
    let output_part_2 = part_2(input);
    println!("Part 2 output is {:?}", output_part_2);
    assert_eq!(1061, output_part_1);
    assert_eq!(1006, output_part_2);
}

fn parse_grid(input: &str) -> Grid {
    let mut grid: Grid = [[false; MAX_WIDTH]; MAX_HEIGHT];
    for (column, line) in input.lines().enumerate() {
        for (row, character) in line.chars().enumerate() {
            grid[column][row] = character == '#';
        }
    }
    grid
}

fn count_on_lights(grid: &Grid) -> usize {
    let mut count = 0;
    for row in grid {
        for light in row {
            if *light {
                count += 1;
            }
        }
    }
    count
}

fn calculate_next_grid(grid: &Grid) -> Grid {
    let mut next_grid = [[false; MAX_WIDTH]; MAX_HEIGHT];

    for row in 0..MAX_WIDTH {
        for column in 0..MAX_HEIGHT {
            let current_light = grid[column][row];

            let mut on_neighbors = 0;
            for column_change in -1..=1 {
                for row_change in -1..=1 {
                    let neighbor_column = column as i64 + column_change;
                    let neighbor_row = row as i64 + row_change;
                    if neighbor_column < 0
                        || neighbor_column >= MAX_HEIGHT as i64
                        || neighbor_row < 0
                        || neighbor_row >= MAX_WIDTH as i64
                        || (neighbor_row as usize, neighbor_column as usize) == (row, column)
                    {
                        continue;
                    }
                    if grid[neighbor_column as usize][neighbor_row as usize] {
                        on_neighbors += 1;
                    }
                }
            }
            let next_light = match current_light {
                true => (on_neighbors == 2 || on_neighbors == 3),
                false => on_neighbors == 3,
            };

            next_grid[column][row] = next_light;
        }
    }

    next_grid
}

fn part_1(input: &str) -> usize {
    let mut grid = parse_grid(input);
    for _ in 0..100 {
        grid = calculate_next_grid(&grid);
    }
    count_on_lights(&grid)
}

fn part_2(input: &str) -> usize {
    let mut grid = parse_grid(input);
    for _ in 0..100 {
        grid = calculate_next_grid(&grid);
        grid[0][0] = true;
        grid[0][99] = true;
        grid[99][0] = true;
        grid[99][99] = true;
    }
    count_on_lights(&grid)
}
