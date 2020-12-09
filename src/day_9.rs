use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[allow(dead_code)]
pub fn day_9() -> io::Result<()> {
    let file = File::open("inputs/day_9")?;
    let reader = BufReader::new(file);
    let numbers: Vec<u64> = reader
        .lines()
        .filter_map(|l| l.map(|l| l.parse().unwrap()).ok())
        .collect();
    let part_1 = part_1(&numbers).unwrap();

    // Part 1
    println!("Day 9\nPart 1: {}", part_1);

    // Part 2
    println!("Part 2: {}\n", part_2(&numbers, *part_1).unwrap());
    Ok(())
}

fn part_1(numbers: &[u64]) -> Option<&u64> {
    let mut prev: VecDeque<&u64> = VecDeque::with_capacity(25);
    numbers.iter().find(|&n| {
        if prev.len() == 25 {
            let found = prev.iter().tuple_combinations().all(|(&x, &y)| x + y != *n);
            prev.pop_front();
            prev.push_back(n);
            found
        } else {
            prev.push_back(n);
            false
        }
    })
}

fn part_2(numbers: &[u64], number: u64) -> Option<u64> {
    (2..numbers.len()).find_map(|n| {
        if let Some(v) = numbers
            .windows(n)
            .find(|slice| slice.iter().sum::<u64>() == number)
        {
            if let MinMax(min, max) = v.iter().minmax() {
                Some(min + max)
            } else {
                None
            }
        } else {
            None
        }
    })
}
