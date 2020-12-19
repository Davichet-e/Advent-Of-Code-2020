use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn day_18() -> io::Result<()> {
    let file = File::open("inputs/day_18")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Part 1
    println!("Day 18\nPart 1: {}", part_1(&lines));

    // Part 2
    println!("Part 2: {}\n", part_2(&lines));
    Ok(())
}

#[derive(Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

impl Operation {
    fn from_char(ch: char) -> Self {
        if ch == '+' {
            Operation::Add
        } else {
            Operation::Multiply
        }
    }

    fn operate(&self, lhs: u64, rhs: u64) -> u64 {
        if let Operation::Add = self {
            lhs + rhs
        } else {
            lhs * rhs
        }
    }
}

fn part_1(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .fold(
                    [(0, Operation::Add)]
                        .iter()
                        .copied()
                        .collect::<VecDeque<(u64, Operation)>>(),
                    |mut stack, ch| {
                        let last = stack.back_mut().unwrap();
                        match ch {
                            '(' => {
                                stack.push_back((0, Operation::Add));
                            }
                            c if c.is_ascii_digit() => {
                                last.0 = last.1.operate(last.0, c.to_digit(10).unwrap() as u64);
                            }
                            ')' => {
                                let d = stack.pop_back().unwrap().0;

                                let last = stack.back_mut().unwrap();
                                last.0 = last.1.operate(last.0, d);
                            }
                            ' ' => (),
                            ch => last.1 = Operation::from_char(ch),
                        }
                        stack
                    },
                )
                .back()
                .unwrap()
                .0
        })
        .sum()
}

fn part_2(lines: &[String]) -> u64 {
    lines
        .iter()
        .map(|l| {
            match l
                .chars()
                .fold(
                    [(0, Operation::Add, None)]
                        .iter()
                        .copied()
                        .collect::<VecDeque<(u64, Operation, Option<u64>)>>(),
                    |mut stack, ch| {
                        let last = stack.back_mut().unwrap();
                        match ch {
                            '(' => {
                                stack.push_back((0, Operation::Add, None));
                            }
                            '+' => last.1 = Operation::Add,
                            '*' => {
                                if let Some(ref mut n) = last.2 {
                                    last.0 *= *n;
                                }
                                last.2 = None;
                                last.1 = Operation::Multiply;
                            }
                            c if c.is_ascii_digit() => {
                                let digit = c.to_digit(10).unwrap() as u64;
                                if let Some(ref mut n) = last.2 {
                                    if let Operation::Add = last.1 {
                                        *n += digit;
                                    } else {
                                        last.0 *= *n;
                                        *n = digit;
                                    }
                                } else if let Operation::Add = last.1 {
                                    last.0 += digit;
                                } else {
                                    last.2 = Some(digit);
                                }
                            }
                            ')' => {
                                let mut d = stack.pop_back().unwrap();
                                if let Some(n) = d.2 {
                                    d.0 *= n;
                                }
                                let last = stack.back_mut().unwrap();
                                if let Operation::Multiply = last.1 {
                                    last.2 = Some(d.0);
                                } else if let Some(n) = last.2 {
                                    last.2 = Some(n + d.0);
                                } else {
                                    last.0 = last.0 + d.0;
                                }
                            }
                            _ => (),
                        }

                        stack
                    },
                )
                .back()
                .unwrap()
            {
                (n, _, Some(u)) => n * u,
                (n, _, _) => *n,
            }
        })
        .sum()
}
