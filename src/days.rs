use crate::days::day01::Day1Solver;
use crate::days::day02::Day2Solver;
use crate::days::DayError::{DaySolutionDoesNotExist, InvalidDay};
use thiserror::Error;
use crate::days::day03::Day3Solver;

mod day01;
mod day02;
mod day03;

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
            Day(day_number) => Err(DaySolutionDoesNotExist(day_number)),
        }
    }
}
