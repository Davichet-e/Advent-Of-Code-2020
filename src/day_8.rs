use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
use std::str::FromStr;

#[allow(dead_code)]
pub fn day_8() -> io::Result<()> {
    let file = File::open("inputs/day_8")?;
    let reader = BufReader::new(file);
    let lines: Vec<Line> = reader
        .lines()
        .enumerate()
        .map(|(i, l)| Line(i, Instruction::from_str(&l.unwrap()).unwrap()))
        .collect();

    // Part 1
    println!("Day 8\nPart 1: {}", part_1(&lines));

    // Part 2
    println!("Part 2: {}\n", part_2(&lines));
    Ok(())
}

struct Line(usize, Instruction);

enum Instruction {
    NOP(i64),
    ACC(i64),
    JMP(i64),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        match split[0] {
            "jmp" => {
                if let Ok(n) = split[1].parse::<i64>() {
                    Ok(Instruction::JMP(n))
                } else {
                    Err("Error parsing the instruction".into())
                }
            }
            "acc" => {
                if let Ok(n) = split[1].parse::<i64>() {
                    Ok(Instruction::ACC(n))
                } else {
                    Err("Error parsing the instruction".into())
                }
            }
            "nop" => {
                if let Ok(n) = split[1].parse::<i64>() {
                    Ok(Instruction::NOP(n))
                } else {
                    Err("Error parsing the instruction".into())
                }
            }
            _ => Err("Error parsing the instruction".into()),
        }
    }
}

fn part_1(lines: &[Line]) -> i64 {
    let mut acc = 0i64;
    let mut i = 0;
    let mut visited_lines = HashSet::new();
    loop {
        let line = &lines[i as usize];
        if visited_lines.contains(&line.0) {
            break acc;
        } else {
            visited_lines.insert(line.0);
            match line {
                Line(_, Instruction::JMP(n)) => i += n,
                Line(_, Instruction::ACC(n)) => {
                    i += 1;
                    acc += n;
                }
                _ => i += 1,
            }
        }
    }
}

fn part_2(lines: &[Line]) -> i64 {
    let mut lines_fil: Vec<&Line> = lines
        .iter()
        .filter(|Line(_, ins)| !matches!(ins, Instruction::ACC(_)))
        .collect();
    let mut try_change = lines_fil.pop().unwrap();
    let mut acc = 0i64;
    let mut i = 0i64;
    let mut visited_lines = HashSet::new();
    while (i as usize) < lines.len() {
        let line = &lines[i as usize];
        if visited_lines.contains(&line.0) {
            visited_lines = HashSet::new();
            try_change = lines_fil.pop().unwrap();
            i = 0;
            acc = 0;
            continue;
        }
        visited_lines.insert(line.0);
        match line {
            Line(l, Instruction::JMP(_)) if *l == try_change.0 => i += 1,
            Line(_, Instruction::JMP(n)) => i += n,

            Line(l, Instruction::NOP(n)) if *l == try_change.0 => i += n,
            Line(_, Instruction::NOP(_)) => i += 1,

            Line(_, Instruction::ACC(n)) => {
                i += 1;
                acc += n;
            }
        }
    }
    acc
}
