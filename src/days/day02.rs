use crate::days::day02::LevelDirection::{Decreasing, Increasing, Stale};
use crate::days::day02::ReportErrors::LevelInputError;
use crate::days::day02::ReportStatus::{Safe, Unsafe};
use crate::days::{DayErrors, DaySolver};
use std::cmp::PartialEq;
use thiserror::Error;

pub struct Day2Solver;

#[derive(Error, Debug)]
enum ReportErrors {
    #[error("Invalid level input {0}")]
    LevelInputError(String),
}

#[derive(PartialEq, Clone, Copy)]
enum LevelDirection {
    Increasing,
    Decreasing,
    Stale,
}

#[derive(PartialEq)]
enum ReportStatus {
    Safe,
    Unsafe,
}

struct Report {
    levels: Vec<i32>,
    status: ReportStatus,
}

fn get_direction_for_levels(level1: &i32, level2: &i32) -> LevelDirection {
    if level1 > level2 {
        return Decreasing;
    }

    if level2 > level1 {
        return Increasing;
    }

    Stale
}

fn is_safe_difference(level1: &i32, level2: &i32) -> bool {
    let difference = (level1 - level2).abs();

    match difference {
        1..=3 => true,
        _ => false,
    }
}

fn get_report_status(levels: &Vec<i32>) -> Result<ReportStatus, ReportErrors> {
    let first_level = levels
        .get(0)
        .ok_or(LevelInputError("No first level in the report".to_string()))?;
    let second_level = levels
        .get(1)
        .ok_or(LevelInputError("No second level in the report".to_string()))?;

    let initial_direction = get_direction_for_levels(first_level, second_level);

    if initial_direction == Stale {
        return Ok(Unsafe);
    }
    for (index, current_level) in levels.iter().enumerate() {
        if let Some(next_level) = levels.get(index + 1) {
            let direction = get_direction_for_levels(current_level, next_level);

            if direction != initial_direction {
                return Ok(Unsafe);
            }

            if !is_safe_difference(current_level, next_level) {
                return Ok(Unsafe);
            }
        }
    }

    Ok(Safe)
}

fn to_report(levels: &Vec<i32>, error_tolerance: &i8) -> Result<Report, ReportErrors> {
    Ok(Report {
        levels: levels.clone(),
        status: get_report_status(levels)?,
    })
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

impl DaySolver for Day2Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayErrors> {
        let error_tolerance: i8 = 0;
        let reports: Vec<Report> = parse_input(input)
            .iter()
            .map(|levels| -> Result<Report, ReportErrors> {
                Ok(to_report(levels, &error_tolerance)?)
            })
            .collect::<Result<Vec<Report>, ReportErrors>>()
            .map_err(|err| DayErrors::InvalidInputError(err.to_string()))?
            .into_iter()
            .filter(|report| report.status == Safe)
            .collect();

        Ok(reports.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayErrors> {
        Ok(String::from("Not Implemented yet"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9\
        "
    }

    #[test]
    fn test_input_parsing() {
        assert_eq!(
            parse_input(get_example_input()),
            vec![
                vec![7, 6, 4, 2, 1],
                vec![1, 2, 7, 8, 9],
                vec![9, 7, 6, 2, 1],
                vec![1, 3, 2, 4, 5],
                vec![8, 6, 4, 4, 1],
                vec![1, 3, 6, 7, 9]
            ]
        )
    }

    #[test]
    fn part1() {
        let solution = Day2Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "2")
    }

    #[test]
    fn part2() {
        let solution = Day2Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "4")
    }
}
