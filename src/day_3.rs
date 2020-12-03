use std::fs::File;
use std::io::{self, prelude::BufRead, BufReader};

pub fn day_3() -> io::Result<()> {
    let file = File::open("inputs/day_3")?;
    let reader = BufReader::new(file);
    let board: &[String] = &reader
        .lines()
        .filter_map(|l| l.ok())
        .collect::<Vec<String>>();

    // Part 1
    println!(
        "Day 3\nPart 1: {}",
        count_trees(&mut [0], &mut vec![3], board)
    );

    //Part 2
    println!(
        "Part 2: {}\n",
        count_trees(&mut [0; 5], &mut vec![1, 3, 5, 7, 1], board)
    );
    Ok(())
}

fn count_trees(acc: &mut [u32], x: &mut Vec<usize>, board: &[String]) -> u64 {
    let str_length = board[0].len();
    let x_ = x.clone();
    let a = (1..board.len()).fold((acc, x, x_), |state, y| {
        for (i, (acc, x)) in state.0.into_iter().zip(state.1.into_iter()).enumerate() {
            if board[match y + 1 {
                _ if i != 4 => y,
                value if value % 2 == 1 || value == board.len() => break,
                value => value,
            }]
            .chars()
            .nth((*x) % str_length)
            .unwrap()
                == '#'
            {
                *acc += 1;
            }
            *x += state.2[i];
        }

        state
    });

    a.0.iter().fold(1, |mut acc, n| {
        acc *= *n as u64;
        acc
    })
}
