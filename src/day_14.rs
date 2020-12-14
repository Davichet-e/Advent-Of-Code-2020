use itertools::{EitherOrBoth, Itertools};
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[allow(dead_code)]
pub fn day_14() -> io::Result<()> {
    let file = File::open("inputs/day_14")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // Part 1
    println!("Day 14\nPart 1: {}", part_1(&lines));

    // Part 2
    println!("Part 2: {}\n", part_2(&lines));
    Ok(())
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\[(\d*)\]").unwrap();
}

fn part_1(lines: &[String]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut actual_bitmask = String::new();
    for line in lines {
        let mut split = line.split(" = ");
        let first: &str = split.next().unwrap();
        let second: &str = split.next().unwrap();
        if first == "mask" {
            actual_bitmask = second.to_owned();
        } else {
            let cap = RE.captures(first).unwrap();
            let key: usize = cap.get(1).unwrap().as_str().parse().unwrap();
            let mut value: usize = second.parse().unwrap();
            actual_bitmask.chars().rev().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    value |= 1 << i;
                } else if c == '0' {
                    value &= !(1 << i);
                }
            });
            memory.insert(key, value);
        }
    }
    memory.values().sum()
}

fn part_2(lines: &[String]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();

    let mut actual_bitmask = String::new();
    for line in lines {
        let mut split = line.split(" = ");
        let first: &str = split.next().unwrap();
        let second: &str = split.next().unwrap();
        if first == "mask" {
            actual_bitmask = String::from(second);
        } else {
            let cap = RE.captures(first).unwrap();
            let value: usize = second.parse().unwrap();
            let mut n = 0;
            let key = format!(
                "{:b}",
                cap.get(1).unwrap().as_str().parse::<usize>().unwrap()
            );

            let key: String = actual_bitmask
                .chars()
                .rev()
                .zip_longest(key.chars().rev())
                .map(|pair| match pair {
                    EitherOrBoth::Both(b, c) => match b {
                        '1' => '1',
                        'X' => {
                            n += 1;
                            'X'
                        }
                        _ => c,
                    },
                    EitherOrBoth::Left(b) => match b {
                        '1' => '1',
                        'X' => {
                            n += 1;
                            'X'
                        }
                        _ => '0',
                    },
                    _ => unreachable!(),
                })
                .collect();

            (0..2usize.pow(n)).for_each(|n| {
                let mut i = 0;
                let key: usize = usize::from_str_radix(
                    &key.chars()
                        .map(|c| {
                            if c == 'X' {
                                i += 1;
                                if (((n) >> (i - 1)) & 1) == 1 {
                                    '1'
                                } else {
                                    '0'
                                }
                            } else {
                                c
                            }
                        })
                        .collect::<String>(),
                    2,
                )
                .unwrap();
                memory.insert(key, value);
            });
        }
    }
    memory.values().sum()
}
