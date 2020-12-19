use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn day_7() -> io::Result<()> {
    let file = File::open("inputs/day_7")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // Part 1
    let bags = part_1(
        &[String::from("shiny gold")].iter().cloned().collect(),
        &lines,
    );
    println!("Day 7\nPart 1: {}", bags.unwrap().len());

    // Part 2
    println!("Part 2: {}\n", part_2("shiny gold", &lines));
    Ok(())
}

fn part_1(bags: &HashSet<String>, lines: &[String]) -> Option<HashSet<String>> {
    let bags_containing_bags: HashSet<String> = lines
        .iter()
        .filter_map(|l| {
            for bag in bags {
                if let Some(v) = l.find(bag) {
                    // If not found in the beginning
                    if v != 0 {
                        return Some(
                            l.split_whitespace()
                                .take(2)
                                .collect::<Vec<&str>>()
                                .join(" "),
                        );
                    }
                }
            }
            None
        })
        .collect();
    if bags_containing_bags.is_empty() {
        None
    // Check if next level (if bags are contained by others bags)
    } else if let Some(bags) = part_1(&bags_containing_bags, lines) {
        Some(bags_containing_bags.union(&bags).cloned().collect())
    } else {
        Some(bags_containing_bags)
    }
}

fn requirements(text: &str) -> Vec<(u16, String)> {
    text.split("contain ")
        .last()
        .unwrap()
        .split(", ")
        .map(|s| {
            let split: Vec<&str> = s.split_whitespace().collect();
            let n: u16 = match split[0].parse() {
                Ok(v) => v,
                Err(_) => 0,
            };
            (n, split[1..split.len() - 1].join(" "))
        })
        .collect()
}

fn part_2(bag: &str, lines: &[String]) -> u16 {
    let requirements = requirements(
        match lines.iter().find(|l| l.split(" bags").any(|s| s == bag)) {
            Some(v) => v,
            None => return 0,
        },
    );
    if requirements.is_empty() {
        0
    } else {
        requirements
            .iter()
            .fold(0, |acc, (n, bag)| acc + n + n * part_2(bag, lines))
    }
}
