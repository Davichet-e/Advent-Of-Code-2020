use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn day_17() -> io::Result<()> {
    let file = File::open("inputs/day_17")?;
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    // Part 1
    println!("Day 17\nPart 1: {}", solve(&lines, false));

    // Part 2
    println!("Part 2: {}\n", solve(&lines, true));

    Ok(())
}

type Coordinate = (i8, i8, i8, i8);

fn parse_input(lines: &[String]) -> HashSet<Coordinate> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(x, line)| {
            line.chars().enumerate().filter_map(move |(y, c)| {
                if c == '#' {
                    Some((x as i8, y as i8, 0, 0))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn solve(lines: &[String], four_dim: bool) -> usize {
    let mut state: HashSet<Coordinate> = parse_input(lines);
    (0..6).for_each(|_| state = step(four_dim, &state));
    state.len()
}

fn step(four_dim: bool, state: &HashSet<Coordinate>) -> HashSet<Coordinate> {
    let mut memory: HashMap<Coordinate, u8> = HashMap::new();
    for (x, y, z, w) in state.iter() {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if four_dim {
                        for dw in -1..=1 {
                            if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                                *memory.entry((x + dx, y + dy, z + dz, w + dw)).or_insert(0) += 1;
                            }
                        }
                    } else if !(dx == 0 && dy == 0 && dz == 0) {
                        *memory.entry((x + dx, y + dy, z + dz, 0)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    memory
        .iter()
        .filter_map(|(point, &n_neighbors)| {
            if n_neighbors == 3 || n_neighbors == 2 && state.contains(point) {
                Some(*point)
            } else {
                None
            }
        })
        .collect()
}
