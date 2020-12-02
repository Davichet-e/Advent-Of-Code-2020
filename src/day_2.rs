use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};
pub fn day_2() -> io::Result<()> {
    let file = File::open("input2")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();
    println!(
        "Day 2\npart 1: {:?}\npart 2: {:?}",
        lines
            .iter()
            .filter(|line| Password::from_text(line).is_valid_part_1())
            .count(),
        lines
            .iter()
            .filter(|line| Password::from_text(line).is_valid_part_2())
            .count()
    );
    Ok(())
}

struct Password<'a> {
    min_max: (usize, usize),
    letter: char,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn from_text(text: &'a str) -> Password {
        let text_split: Vec<&str> = text.split(": ").collect();

        let split: Vec<&str> = text_split[0].split(" ").collect();
        let letter = split[1].chars().next().unwrap();
        let numbers: Vec<usize> = split[0]
            .split("-")
            .filter_map(|n| n.parse::<usize>().ok())
            .collect();
        let min_max = (numbers[0], numbers[1]);
        let password = text_split[1];

        Password {
            min_max,
            letter,
            password,
        }
    }

    fn is_valid_part_1(self) -> bool {
        let n = self.password.matches(self.letter).count();
        n >= self.min_max.0 && n <= self.min_max.1
    }

    fn is_valid_part_2(self) -> bool {
        (self.password.chars().nth(self.min_max.0 - 1).unwrap() == self.letter)
            ^ (self.password.chars().nth(self.min_max.1 - 1).unwrap() == self.letter)
    }
}