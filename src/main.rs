// use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

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
fn main() -> io::Result<()> {
    let file = File::open("input2")?;
    let reader = BufReader::new(file);
    println!(
        "{:?}",
        reader
            .lines()
            .filter_map(|l| l.ok())
            .filter(|line| Password::from_text(line).is_valid_part_2())
            .count()
    );
    // let file = File::open("input")?;
    // let reader = BufReader::new(file);
    // let lines_bis: &[u32] = &reader
    //     .lines()
    //     .filter_map(|l| l.unwrap().parse::<u32>().ok())
    //     .collect::<Vec<u32>>();
    // 'outer: for (i, &n) in lines_bis.iter().enumerate() {
    //     for (j, &n_bis) in lines_bis.iter().enumerate().skip(i) {
    //         for &n_bis_bis in lines_bis.iter().skip(j) {
    //             if n + n_bis + n_bis_bis == 2020 {
    //                 println!("{}", n * n_bis * n_bis_bis);
    //                 break 'outer;
    //             }
    //         }
    //     }
    // }

    // println!(
    //     "{}",
    //     reader
    //         .lines()
    //         .filter_map(|result| result.unwrap().parse::<u32>().ok())
    //         .collect::<Vec<u32>>()
    //         .iter()
    //         .tuple_combinations::<(_, _, _)>()
    //         .find(|(&x, &y, &c)| x + y + c == 2020)
    //         .map_or(0, |(a, b, c)| a * b * c)
    // );

    Ok(())
}
