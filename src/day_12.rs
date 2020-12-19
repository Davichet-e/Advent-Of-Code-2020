use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[allow(dead_code)]
pub fn day_12() -> io::Result<()> {
    let file = File::open("inputs/day_12")?;
    let reader = BufReader::new(file);
    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|l| Instruction::from_text(&l.unwrap()))
        .collect();
    // Part 1
    let mut ship_coordinates = ShipCoordinates::new();

    instructions
        .clone()
        .into_iter()
        .for_each(|instruction| ship_coordinates.turn_1(instruction));

    println!(
        "Day 12\nPart 1: {}",
        ship_coordinates.distance_from_origin()
    );

    // Part 2
    ship_coordinates = ShipCoordinates::new();
    instructions
        .into_iter()
        .for_each(|instruction| ship_coordinates.turn_2(instruction));

    println!("Part 2: {}\n", ship_coordinates.distance_from_origin());

    Ok(())
}

struct ShipCoordinates {
    east: i32,
    north: i32,
    waypoint: Waypoint,
    actual_direction: Direction,
}

impl ShipCoordinates {
    fn new() -> Self {
        ShipCoordinates {
            east: 0,
            north: 0,
            waypoint: Waypoint(10, 1),
            actual_direction: Direction::EAST,
        }
    }
    fn turn_1(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(degrees) => match self.actual_direction {
                Direction::NORTH => self.north += degrees as i32,
                Direction::SOUTH => self.north -= degrees as i32,
                Direction::EAST => self.east += degrees as i32,
                Direction::WEST => self.east -= degrees as i32,
            },
            Instruction::North(degrees) => self.north += degrees as i32,
            Instruction::South(degrees) => self.north -= degrees as i32,
            Instruction::East(degrees) => self.east += degrees as i32,
            Instruction::West(degrees) => self.east -= degrees as i32,
            instruction => self.actual_direction.turn(instruction),
        }
    }

    fn turn_2(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Forward(degrees) => {
                self.east += self.waypoint.0 * degrees as i32;
                self.north += self.waypoint.1 * degrees as i32;
            }
            Instruction::North(degrees) => self.waypoint.1 += degrees as i32,
            Instruction::South(degrees) => self.waypoint.1 -= degrees as i32,
            Instruction::East(degrees) => self.waypoint.0 += degrees as i32,
            Instruction::West(degrees) => self.waypoint.0 -= degrees as i32,
            instruction => self.waypoint.turn(instruction),
        }
    }

    fn distance_from_origin(&self) -> u16 {
        (self.east.abs() + self.north.abs()) as u16
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Left(u16),
    Right(u16),
    Forward(u16),
}

impl Instruction {
    fn from_text(text: &str) -> Self {
        let (first, second) = text.split_at(1);
        let degrees: u16 = second.parse().unwrap();
        match first {
            "N" => Instruction::North(degrees),
            "E" => Instruction::East(degrees),
            "S" => Instruction::South(degrees),
            "W" => Instruction::West(degrees),
            "L" => Instruction::Left(degrees),
            "R" => Instruction::Right(degrees),
            "F" => Instruction::Forward(degrees),
            _ => unreachable!(),
        }
    }
}

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl Direction {
    fn turn(&mut self, direction: Instruction) {
        use Direction::*;
        use Instruction::{Left, Right};

        *self = match (&self, direction) {
            (NORTH, Left(90)) | (NORTH, Right(270)) => WEST,
            (NORTH, Left(180)) | (NORTH, Right(180)) => SOUTH,
            (NORTH, Right(90)) | (NORTH, Left(270)) => EAST,

            (SOUTH, Left(90)) | (SOUTH, Right(270)) => EAST,
            (SOUTH, Left(180)) | (SOUTH, Right(180)) => NORTH,
            (SOUTH, Right(90)) | (SOUTH, Left(270)) => WEST,

            (EAST, Left(90)) | (EAST, Right(270)) => NORTH,
            (EAST, Left(180)) | (EAST, Right(180)) => WEST,
            (EAST, Right(90)) | (EAST, Left(270)) => SOUTH,

            (WEST, Left(90)) | (WEST, Right(270)) => SOUTH,
            (WEST, Left(180)) | (WEST, Right(180)) => EAST,
            (WEST, Right(90)) | (WEST, Left(270)) => NORTH,

            _ => unreachable!(),
        }
    }
}

struct Waypoint(i32, i32); // (east, north)

impl Waypoint {
    fn turn(&mut self, direction: Instruction) {
        use Instruction::*;

        *self = match direction {
            Right(90) | Left(270) => Waypoint(self.1, -self.0),
            Right(180) | Left(180) => Waypoint(-self.0, -self.1),
            Left(90) | Right(270) => Waypoint(-self.1, self.0),
            _ => unreachable!(),
        }
    }
}
