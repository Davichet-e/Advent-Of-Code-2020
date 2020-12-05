use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Debug)]
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
        let seat = isize::from_str_radix(
            &*ZEROS_RE.replace_all(&*ONES_RE.replace_all(string, "1"), "0"),
            2,
        )
        .unwrap() as i16;
        Seat {
            column: (seat as i16 & 7) as i8,
            row: (seat as i16 >> 3) as i8,
        }
    }

    fn id(self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

fn not_consecutive_items(list: &[u16]) -> u16 {
    list.iter()
        .take(list.len() - 1)
        .zip(list.iter().skip(1))
        .find(|(&l, &u)| u == (l + 2))
        .unwrap()
        .0
        + 1
}

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
    let my_id = not_consecutive_items(&ids);
    println!("Part 2: {:?}\n", my_id);

    Ok(())
}
