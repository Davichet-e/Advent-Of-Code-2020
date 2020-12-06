use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[allow(dead_code)]
pub fn day_5() -> io::Result<()> {
    let file = File::open("inputs/day_5")?;
    let reader = BufReader::new(file);
    let mut ids = reader
        .lines()
        .map(|l| Seat::new(&l.unwrap()).id())
        .collect::<Vec<u16>>();
    ids.sort();

    // Part 1
    println!("Day 5\nPart 1: {:?}", ids.last().unwrap());

    // Part 2
    let my_id = find_id(&mut ids);
    println!("Part 2: {:?}\n", my_id);

    Ok(())
}

struct Seat {
    row: i8,
    column: i8,
}

impl Seat {
    fn new(string: &str) -> Self {
        lazy_static! {
            static ref ZEROS_RE: Regex = Regex::new("[FL]").unwrap();
            static ref ONES_RE: Regex = Regex::new("[BR]").unwrap();
        }
        let seat = i16::from_str_radix(
            &*ZEROS_RE.replace_all(&*ONES_RE.replace_all(string, "1"), "0"),
            2,
        )
        .unwrap();
        Seat {
            column: (seat & 7) as i8,
            row: (seat >> 3) as i8,
        }
    }

    fn id(self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

fn find_id(list: &[u16]) -> u16 {
    // list should be sorted
    *(1..*list.last().unwrap())
        .collect::<HashSet<u16>>()
        .difference(&list.iter().cloned().collect())
        .next()
        .unwrap()
}
