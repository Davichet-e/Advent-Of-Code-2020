use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

#[derive(Debug)]
struct Seat {
    row: i8,
    column: i8,
}

impl Seat {
    fn new(string: &str) -> Self {
        let ((row, _), (column, _)): ((i8, i8), (i8, i8)) = string.chars().fold(
            ((0, 127), (0, 7)),
            |((min_row, max_row), (min_col, max_col)), c| match c {
                'F' => (
                    (
                        min_row,
                        max_row - ((max_row as f32 - min_row as f32) / 2.0).trunc() as i8,
                    ),
                    (min_col, max_col),
                ),
                'B' => (
                    (
                        min_row + ((max_row as f32 - min_row as f32) as f32 / 2.0).ceil() as i8,
                        max_row,
                    ),
                    (min_col, max_col),
                ),
                'L' => (
                    (min_row, max_row),
                    (
                        min_col,
                        max_col - ((max_col as f32 - min_col as f32) / 2.0).trunc() as i8,
                    ),
                ),
                'R' => (
                    (min_row, max_row),
                    (
                        min_col + ((max_col as f32 - min_col as f32) as f32 / 2.0).ceil() as i8,
                        max_col,
                    ),
                ),
                _ => unreachable!(),
            },
        );
        Seat { column, row }
    }

    fn id(self) -> u16 {
        (self.row as u16) * 8 + (self.column as u16)
    }
}

fn not_consecutive_items(list: &[u16]) -> Vec<u16> {
    let mut first: usize = 0;
    let last: usize = list.len();
    let mut not_sorted = Vec::new();

    let mut next: usize = first + 1;
    while next != last {
        if list[next] - list[first] != 1 {
            not_sorted.push(list[first] + 1);
        };
        first += 1;
        next += 1;
    }
    not_sorted
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
    let ids_not_consecutive = not_consecutive_items(&ids);
    println!("Part 2: {:?}\n", ids_not_consecutive[0]);

    Ok(())
}
