use crate::days::day01::Day1Solver;
use crate::days::Day::Day2;
use crate::define_advent_days;
use thiserror::Error;
use crate::days::day02::Day2Solver;

pub mod day01;
mod day02;
mod day_macro;

#[derive(Error, Debug)]
pub enum DayErrors {
    #[error("Day number {0} is not implemented yet.")]
    DayDoesNotExist(u8),

    #[error("Received invalid input for day: {0}")]
    InvalidInputError(String),
}

define_advent_days!(Day1, Day2);

pub trait DaySolver {
    fn solve_part1(&self, input: &str) -> Result<String, DayErrors>;
    fn solve_part2(&self, input: &str) -> Result<String, DayErrors>;
}

impl Day {
    pub fn to_solver(&self) -> Box<dyn DaySolver> {
        match self {
            Day::Day1 => Box::new(Day1Solver),
            Day::Day2 => Box::new(Day2Solver),
        }
    }
}
