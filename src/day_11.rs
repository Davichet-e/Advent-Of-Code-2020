use std::{fs, io};

#[allow(dead_code)]
pub fn day_11() -> io::Result<()> {
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
        (seat_not_occupied(index as i32 + 1, matrix))
            && (seat_not_occupied(index as i32 + 99, matrix))
            && (seat_not_occupied(index as i32 - 1, matrix))
            && (seat_not_occupied(index as i32 - 99, matrix))
            && (seat_not_occupied(index as i32 + 100, matrix))
            && (seat_not_occupied(index as i32 + 98, matrix))
            && (seat_not_occupied(index as i32 - 100, matrix))
            && (seat_not_occupied(index as i32 - 98, matrix))
    };

    let crowded_1 = |index: usize, matrix: &[Seat]| {
        (!seat_not_occupied((index) as i32 + 1, matrix)) as u8
            + (!seat_not_occupied(index as i32 + 99, matrix)) as u8
            + (!seat_not_occupied(index as i32 - 1, matrix)) as u8
            + (!seat_not_occupied(index as i32 - 99, matrix)) as u8
            + (!seat_not_occupied(index as i32 + 100, matrix)) as u8
            + (!seat_not_occupied(index as i32 + 98, matrix)) as u8
            + (!seat_not_occupied(index as i32 - 100, matrix)) as u8
            + (!seat_not_occupied(index as i32 - 98, matrix)) as u8
            >= 4
    };

    let iteration_1: Box<dyn Fn((usize, Seat, &[Seat])) -> Seat> =
        Box::new(|(i, seat, matrix): (usize, Seat, &[Seat])| match seat {
            Seat::FREE if not_occupied(i, matrix) => Seat::OCCUPIED,
            Seat::OCCUPIED if crowded_1(i, matrix) => Seat::FREE,
            _ => seat,
        });

    // Part 2
    let seat_not_occupied_direction = |mut index: i32, matrix: &[Seat], direction: Direction| loop {
        index += i32::from(&direction);
        if is_overflow(index) {
            break true;
        } else {
            match matrix[index as usize] {
                Seat::FREE => break true,
                Seat::OCCUPIED => break false,
                Seat::ENDLINE => break true,
                _ => (),
            }
        }
    };

    let crowded_2 = |index: usize, matrix: &[Seat]| {
        (!(seat_not_occupied_direction(index as i32, matrix, Direction::UP)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::UP_LEFT)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::UP_RIGHT)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::DOWN)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::DOWN_LEFT)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::DOWN_RIGHT)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::LEFT)) as u8)
            + (!(seat_not_occupied_direction(index as i32, matrix, Direction::RIGHT)) as u8)
            >= 5
    };

    let not_occupied_2 = |index: usize, matrix: &[Seat]| {
        (seat_not_occupied_direction(index as i32, matrix, Direction::UP))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::UP_LEFT))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::UP_RIGHT))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::DOWN))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::DOWN_LEFT))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::DOWN_RIGHT))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::LEFT))
            && (seat_not_occupied_direction(index as i32, matrix, Direction::RIGHT))
    };

    let iteration_2: Box<dyn Fn((usize, Seat, &[Seat])) -> Seat> =
        Box::new(|(i, seat, matrix): (usize, Seat, &[Seat])| match seat {
            Seat::FREE if not_occupied_2(i, matrix) => Seat::OCCUPIED,
            Seat::OCCUPIED if crowded_2(i, matrix) => Seat::FREE,
            _ => seat,
        });

    for iteration in [iteration_1, iteration_2].iter() {
        let mut prev_iter: Vec<Seat> = matrix
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, s)| iteration((i, s, &matrix)))
            .collect::<Vec<_>>();

        let not_changing = loop {
            let new_iter: Vec<Seat> = prev_iter
                .iter()
                .enumerate()
                .map(|(i, seat)| iteration((i, seat.clone(), &prev_iter)))
                .collect::<Vec<_>>();
            if prev_iter == new_iter {
                break new_iter;
            } else {
                prev_iter = new_iter;
            }
        };
        println!(
            "{}",
            not_changing
                .iter()
                .filter(|seat| matches!(seat, Seat::OCCUPIED))
                .count()
        );
    }
    Ok(())
}

#[derive(PartialEq, Clone)]
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

impl From<&Direction> for i32 {
    fn from(item: &Direction) -> Self {
        match item {
            Direction::UP => -99,
            Direction::UP_LEFT => -100,
            Direction::UP_RIGHT => -98,
            Direction::DOWN => 99,
            Direction::DOWN_LEFT => 98,
            Direction::DOWN_RIGHT => 100,
            Direction::LEFT => -1,
            Direction::RIGHT => 1,
        }
    }
}
