use std::collections::HashSet;
use std::fs::File;
use std::io::{self, Read};

#[allow(dead_code)]
pub fn day_6() -> io::Result<()> {
    let mut file = File::open("inputs/day_6")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    // Part 1
    println!("Day 6\nPart 1: {}", part_1(&content));

    // Part 2
    println!("Part 2: {}\n", part_2(&content));
    Ok(())
}

fn part_1(content: &str) -> usize {
    content
        .split("\n\n")
        .map(|s| {
            let set: HashSet<char> = s.chars().collect();
            if set.contains(&'\n') {
                set.len() - 1
            } else {
                set.len()
            }
        })
        .sum()
}

fn part_2(content: &str) -> usize {
    content
        .split("\n\n")
        .map(|s| {
            let mut split = s.split('\n');
            let first_element: HashSet<char> = split.next().unwrap().chars().collect();
            split
                .fold(first_element, |acc_set, string| {
                    acc_set
                        .intersection(&string.chars().collect::<HashSet<_>>())
                        .cloned()
                        .collect()
                })
                .len()
        })
        .sum()
}
