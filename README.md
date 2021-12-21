# Overview

My attempt to solve all AoC puzzles to get proficient writing Rust.

## Project structure

The main package is a [workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html) broken down into sub-packages for each year.
Inside a year's package, each day is a binary crate which will execute both part 1 and 2 for that day.

For example, you can run day16 for 2021 with:

```
cargo run -p aoc-2021 --bin day16
```
(note that single digit days are 0-padded, e.g. `day04`)

## Current status

TBD
