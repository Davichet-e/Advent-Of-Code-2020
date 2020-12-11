use std::{fs, io};

#[allow(dead_code)]
pub fn day_11() -> io::Result<()> {
    use Direction::*;

    let content = fs::read_to_string("inputs/day_11")?;
    let matrix: Vec<Seat> = content
        .chars()
        .map(|c| match c {
            'L' => Seat::FREE,
            '#' => Seat::OCCUPIED,
            '.' => Seat::FLOOR,
            '\n' => Seat::ENDLINE,
            _ => unreachable!(),
        })
        .collect();

    let is_overflow = |index: i32| index < 0 || index as usize >= matrix.len();

    // Part 1
    let seat_not_occupied = |index: i32, matrix: &[Seat]| {
        is_overflow(index) || matrix[index as usize] != Seat::OCCUPIED
    };

    let not_occupied = |index: usize, matrix: &[Seat]| {
        (seat_not_occupied(index + &RIGHT, matrix))
            && (seat_not_occupied(index + &DOWN, matrix))
            && (seat_not_occupied(index + &LEFT, matrix))
            && (seat_not_occupied(index + &UP, matrix))
            && (seat_not_occupied(index + &DOWN_RIGHT, matrix))
            && (seat_not_occupied(index + &DOWN_LEFT, matrix))
            && (seat_not_occupied(index + &UP_RIGHT, matrix))
            && (seat_not_occupied(index + &UP_LEFT, matrix))
    };

    let crowded_1 = |index: usize, matrix: &[Seat]| {
        (!seat_not_occupied(index + &RIGHT, matrix)) as u8
            + (!seat_not_occupied(index + &DOWN, matrix)) as u8
            + (!seat_not_occupied(index + &LEFT, matrix)) as u8
            + (!seat_not_occupied(index + &UP, matrix)) as u8
            + (!seat_not_occupied(index + &DOWN_RIGHT, matrix)) as u8
            + (!seat_not_occupied(index + &DOWN_LEFT, matrix)) as u8
            + (!seat_not_occupied(index + &UP_RIGHT, matrix)) as u8
            + (!seat_not_occupied(index + &UP_LEFT, matrix)) as u8
            >= 4
    };

    let iteration_1: Box<dyn Fn((usize, &Seat, &[Seat])) -> Seat> =
        Box::new(|(i, seat, matrix)| match seat {
            Seat::FREE if not_occupied(i, matrix) => Seat::OCCUPIED,
            Seat::OCCUPIED if crowded_1(i, matrix) => Seat::FREE,
            _ => *seat,
        });

    // Part 2
    let seat_not_occupied_2 = |index: usize, matrix: &[Seat], direction: Direction| {
        let mut acc: i32 = index as i32;
        loop {
            acc = acc as usize + &direction;
            if is_overflow(acc) {
                break true;
            } else {
                match matrix[acc as usize] {
                    Seat::FREE => break true,
                    Seat::OCCUPIED => break false,
                    Seat::ENDLINE => break true,
                    _ => (),
                }
            }
        }
    };

    let crowded_2 = |index: usize, matrix: &[Seat]| {
        (!(seat_not_occupied_2(index, matrix, UP)) as u8)
            + (!(seat_not_occupied_2(index, matrix, UP_LEFT)) as u8)
            + (!(seat_not_occupied_2(index, matrix, UP_RIGHT)) as u8)
            + (!(seat_not_occupied_2(index, matrix, DOWN)) as u8)
            + (!(seat_not_occupied_2(index, matrix, DOWN_LEFT)) as u8)
            + (!(seat_not_occupied_2(index, matrix, DOWN_RIGHT)) as u8)
            + (!(seat_not_occupied_2(index, matrix, LEFT)) as u8)
            + (!(seat_not_occupied_2(index, matrix, RIGHT)) as u8)
            >= 5
    };

    let not_occupied_2 = |index: usize, matrix: &[Seat]| {
        (seat_not_occupied_2(index, matrix, UP))
            && (seat_not_occupied_2(index, matrix, UP_LEFT))
            && (seat_not_occupied_2(index, matrix, UP_RIGHT))
            && (seat_not_occupied_2(index, matrix, DOWN))
            && (seat_not_occupied_2(index, matrix, DOWN_LEFT))
            && (seat_not_occupied_2(index, matrix, DOWN_RIGHT))
            && (seat_not_occupied_2(index, matrix, LEFT))
            && (seat_not_occupied_2(index, matrix, RIGHT))
    };

    let iteration_2: Box<dyn Fn((usize, &Seat, &[Seat])) -> Seat> =
        Box::new(|(i, seat, matrix)| match seat {
            Seat::FREE if not_occupied_2(i, matrix) => Seat::OCCUPIED,
            Seat::OCCUPIED if crowded_2(i, matrix) => Seat::FREE,
            _ => *seat,
        });

    for (i, iteration) in [iteration_1, iteration_2].iter().enumerate() {
        let mut prev_iter: Vec<Seat> = matrix
            .iter()
            .enumerate()
            .map(|(i, s)| iteration((i, s, &matrix)))
            .collect();

        let not_changing = loop {
            let new_iter: Vec<Seat> = prev_iter
                .iter()
                .enumerate()
                .map(|(i, seat)| iteration((i, seat, &prev_iter)))
                .collect();
            if prev_iter == new_iter {
                break new_iter;
            } else {
                prev_iter = new_iter;
            }
        };

        let n_of_occupies = not_changing
            .iter()
            .filter(|seat| matches!(seat, Seat::OCCUPIED))
            .count();
        if i == 0 {
            println!("Day 11\nPart 1: {}", n_of_occupies);
        } else {
            println!("Part 2: {}\n", n_of_occupies);
        }
    }
    Ok(())
}

#[derive(PartialEq, Clone, Copy)]
enum Seat {
    FREE,
    FLOOR,
    OCCUPIED,
    ENDLINE,
}

#[allow(non_camel_case_types)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    UP_LEFT,
    DOWN_RIGHT,
    DOWN_LEFT,
    UP_RIGHT,
}

impl std::ops::Add<&Direction> for usize {
    type Output = i32;
    fn add(self, rhs: &Direction) -> i32 {
        match rhs {
            Direction::UP => self as i32 - 99,
            Direction::UP_LEFT => self as i32 - 100,
            Direction::UP_RIGHT => self as i32 - 98,
            Direction::DOWN => self as i32 + 99,
            Direction::DOWN_LEFT => self as i32 + 98,
            Direction::DOWN_RIGHT => self as i32 + 100,
            Direction::LEFT => self as i32 - 1,
            Direction::RIGHT => self as i32 + 1,
        }
    }
}
