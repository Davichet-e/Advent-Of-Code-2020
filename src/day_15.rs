use std::collections::HashMap;
use std::{fs, io};

#[allow(dead_code)]
pub fn day_15() -> io::Result<()> {
    let input: Vec<usize> = fs::read_to_string("inputs/day_15")?
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let last_value = *input.last().unwrap();
    let length = input.len();

    let initial_turns: HashMap<usize, usize> = input
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i + 1))
        .collect();

    // Part 1
    println!(
        "Day 15\nPart 1: {}",
        solve(2020, length, last_value, &initial_turns)
    );

    // Part 2
    println!(
        "Part 2: {}\n",
        solve(30000000, length, last_value, &initial_turns)
    );
    Ok(())
}

fn solve(
    nth: usize,
    length: usize,
    last_value: usize,
    initial_turns: &HashMap<usize, usize>,
) -> usize {
    let mut memory = initial_turns.clone();
    memory.reserve(nth / 8);

    (length..nth).fold(last_value, |n, i| {
        if let Some(v) = memory.insert(n, i) {
            i - v
        } else {
            0
        }
    })
}
