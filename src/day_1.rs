use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[allow(dead_code)]
pub fn day_1() -> io::Result<()> {
    let file = File::open("inputs/day_1")?;
    let reader = BufReader::new(file);
    let numbers: Vec<u32> = reader
        .lines()
        .filter_map(|result| result.unwrap().parse().ok())
        .collect();
    println!(
        "Day 1\npart 1: {}",
        numbers
            .iter()
            .tuple_combinations()
            .find(|(&x, &y)| x + y == 2020)
            .map_or(0, |(a, b)| a * b)
    );
    println!(
        "part 2: {}\n",
        numbers
            .iter()
            .tuple_combinations::<(_, _, _)>()
            .find(|(&x, &y, &c)| x + y + c == 2020)
            .map_or(0, |(a, b, c)| a * b * c)
    );
    Ok(())
}
