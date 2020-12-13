use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

fn find_inverse(ni: u64, modulo: u64) -> u64 {
    (1..).find(|n| (n * ni) % modulo == 1).unwrap()
}

#[allow(dead_code)]
pub fn day_13() -> io::Result<()> {
    let file = File::open("inputs/day_13")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let early_timestamp: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    let buses: Vec<u32> = lines
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .filter_map(|c| {
            if c != "x" {
                Some(c.parse().unwrap())
            } else {
                None
            }
        })
        .collect();
    // Part 1
    let part_1 = buses
        .iter()
        .map(|number| {
            let quotient = early_timestamp / number + 1;
            (number * quotient - early_timestamp, number)
        })
        .min()
        .unwrap();

    println!("Day 13\nPart 1: {}", part_1.0 * part_1.1);

    // Part 2
    let big_n: u64 = buses.iter().map(|n| *n as u64).product();

    let part_2: u64 = buses
        .iter()
        .enumerate()
        .map(|(b, n)| {
            let ni = big_n / (*n as u64);
            b as u64 * ni * find_inverse(ni, *n as u64)
        })
        .sum();

    println!("Part 2: {}\n", big_n - part_2 % big_n);
    Ok(())
}
