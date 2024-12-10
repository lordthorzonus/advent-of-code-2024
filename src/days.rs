use crate::days::day01::Day1Solver;
use crate::days::day02::Day2Solver;
use crate::days::DayError::{DaySolutionDoesNotExist, InvalidDay};
use thiserror::Error;
use crate::days::day03::Day3Solver;
use crate::days::day04::Day4Solver;
use crate::days::day05::Day5Solver;
use crate::days::day06::Day6Solver;
use crate::days::day07::Day7Solver;
use crate::days::day08::Day8Solver;
use crate::days::day09::Day9Solver;
use crate::days::day10::Day10Solver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

#[derive(Error, Debug)]
pub enum DayError {
    #[error("Day number {0} is not implemented yet.")]
    DaySolutionDoesNotExist(u8),

    #[error("There is no day number {0} in advent calendar of December.")]
    InvalidDay(u8),

    #[error("Received invalid input for day: {0}")]
    InvalidInputError(String),

    #[error("Unknown error from day solution: '{0}'")]
    Unknown(String)
}

pub struct Day(u8);

impl TryFrom<u8> for Day {
    type Error = DayError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1..=24 => Ok(Day(value)),
            _ => Err(InvalidDay(value)),
        }
    }
}

pub trait DaySolver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError>;
    fn solve_part2(&self, input: &str) -> Result<String, DayError>;
}

impl TryFrom<Day> for Box<dyn DaySolver> {
    type Error = DayError;

    fn try_from(value: Day) -> Result<Self, Self::Error> {
        match value {
            Day(1) => Ok(Box::new(Day1Solver)),
            Day(2) => Ok(Box::new(Day2Solver)),
            Day(3) => Ok(Box::new(Day3Solver)),
            Day(4) => Ok(Box::new(Day4Solver)),
            Day(5) => Ok(Box::new(Day5Solver)),
            Day(6) => Ok(Box::new(Day6Solver)),
            Day(7) => Ok(Box::new(Day7Solver)),
            Day(8) => Ok(Box::new(Day8Solver)),
            Day(9) => Ok(Box::new(Day9Solver)),
            Day(10) => Ok(Box::new(Day10Solver)),
            Day(day_number) => Err(DaySolutionDoesNotExist(day_number)),
        }
    }
}
