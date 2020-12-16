use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;
use std::{fs, io};

type Restriction<'a> = (&'a str, (RangeInclusive<usize>, RangeInclusive<usize>));

#[allow(dead_code)]
pub fn day_16() -> io::Result<()> {
    let content = fs::read_to_string("inputs/day_16")?;
    let mut split = content.split("\n\n");
    let restrictions: Vec<Restriction> = split
        .next()
        .unwrap()
        .split('\n')
        .map(|r| {
            let mut split = r.split(": ");
            let key = split.next().unwrap();
            let mut split = split.next().unwrap().split(" or ");
            let range_1 = {
                let mut v = split.next().unwrap().split('-');
                v.next().unwrap().parse().unwrap()..=v.next().unwrap().parse().unwrap()
            };
            let range_2 = {
                let mut v = split.next().unwrap().split('-');
                v.next().unwrap().parse().unwrap()..=v.next().unwrap().parse().unwrap()
            };
            (key, (range_1, range_2))
        })
        .collect();

    let my_ticket: Vec<usize> = split
        .next()
        .unwrap()
        .split('\n')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let tickets: Vec<Vec<usize>> = split
        .next()
        .unwrap()
        .trim_end()
        .split('\n')
        .skip(1)
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let valid_nearby_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .filter(|t| {
            t.iter().all(|n| {
                restrictions
                    .iter()
                    .any(|(_, (r1, r2))| r1.contains(n) || r2.contains(n))
            })
        })
        .cloned()
        .collect();

    // Part 1
    println!("Day 16\nPart 1: {}", part_1(&tickets, &restrictions));

    // Part 2
    println!(
        "Part 2: {}\n",
        part_2(&valid_nearby_tickets, &restrictions, &my_ticket)
    );
    Ok(())
}

fn part_1(tickets: &[Vec<usize>], restrictions: &[Restriction]) -> usize {
    tickets
        .iter()
        .flatten()
        .filter(|n| {
            !restrictions
                .iter()
                .any(|(_, (r1, r2))| r1.contains(n) || r2.contains(n))
        })
        .sum::<usize>()
}

fn part_2(
    nearby_tickets: &[Vec<usize>],
    restrictions: &[Restriction],
    my_ticket: &[usize],
) -> usize {
    let mut possible_fields: Vec<HashSet<usize>> =
        vec![(0..restrictions.len()).collect(); restrictions.len()];

    for ticket in nearby_tickets {
        for (field_id, field) in ticket.iter().enumerate() {
            for (rule_id, rule) in restrictions.iter().enumerate() {
                if !(rule.1 .0.contains(&field) || rule.1 .1.contains(&field)) {
                    possible_fields[field_id].remove(&rule_id);
                }
            }
        }
    }

    let mut field_to_rule_mapping = HashMap::new();
    while field_to_rule_mapping.len() != restrictions.len() {
        for id in 0..restrictions.len() {
            if possible_fields[id].len() == 1 {
                let rule_id = *possible_fields[id].iter().next().unwrap();
                field_to_rule_mapping.insert(id, rule_id);

                for i in possible_fields.iter_mut() {
                    i.remove(&rule_id);
                }
            }
        }
    }
    my_ticket.iter().enumerate().fold(1, |acc, (i, field)| {
        if field_to_rule_mapping.get(&i).map_or(false, |&id| id < 6) {
            acc * field
        } else {
            acc
        }
    })
}
