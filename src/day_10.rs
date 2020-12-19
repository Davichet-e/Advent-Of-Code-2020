use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn day_10() -> io::Result<()> {
    let file = File::open("inputs/day_10")?;
    let reader = BufReader::new(file);
    let numbers: &[u16] = &{
        let mut numbers: Vec<u16> = reader
            .lines()
            .filter_map(|l| l.map(|l| l.parse().unwrap()).ok())
            .collect();
        numbers.push(0);
        numbers.sort_unstable();
        numbers.push(numbers.last().unwrap() + 3);
        numbers
    };

    // Part 1
    println!("Day 10\nPart 1: {}", part_1(numbers));

    // Part 2
    println!(
        "Part 2: {:?}\n",
        part_2(
            &numbers.iter().collect(),
            0,
            *numbers.last().unwrap(),
            &mut HashMap::new()
        )
    );
    Ok(())
}

fn part_1(numbers: &[u16]) -> u16 {
    let solution = numbers
        .windows(2)
        .fold((0, 0), |state, w| match w[1] - w[0] {
            1 => (state.0 + 1, state.1),
            3 => (state.0, state.1 + 1),
            _ => unreachable!(),
        });
    solution.0 * solution.1
}

fn part_2(numbers: &HashSet<&u16>, number: u16, max: u16, memory: &mut HashMap<u16, u64>) -> u64 {
    if number == max {
        1
    } else {
        ((number + 1)..(number + 4))
            .filter_map(|n| {
                if numbers.contains(&n) {
                    Some(if memory.contains_key(&n) {
                        *memory.get(&n).unwrap()
                    } else {
                        let result = part_2(numbers, n, max, memory);
                        memory.insert(n, result);
                        result
                    })
                } else {
                    None
                }
            })
            .sum()
    }
}
