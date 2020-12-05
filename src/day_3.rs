use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[allow(dead_code)]
pub fn day_3() -> io::Result<()> {
    let file = File::open("inputs/day_3")?;
    let reader = BufReader::new(file);
    let board: &[String] = &reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();
    let mut x1 = count_trees(3, 1, board);
    // Part 1
    println!("Day 3\nPart 1: {}", x1);

    x1 *= count_trees(1, 1, board);
    x1 *= count_trees(5, 1, board);
    x1 *= count_trees(7, 1, board);
    x1 *= count_trees(1, 2, board);

    //Part 2
    println!("Part 2: {}\n", x1);
    Ok(())
}

fn count_trees(x: usize, y: usize, board: &[String]) -> u64 {
    let str_length = board[0].len();
    board
        .iter()
        .step_by(y)
        .fold((0, x), |state, line| {
            if line.chars().nth(state.1 % str_length).unwrap() == '#' {
                (state.0 + 1, state.1 + x)
            } else {
                (state.0, state.1 + x)
            }
        })
        .0
}
