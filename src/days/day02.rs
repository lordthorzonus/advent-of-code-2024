use crate::days::day02::LevelDirection::{Decreasing, Increasing, Stale};
use crate::days::day02::ReportError::{LevelInputError, UnsafeLevelTransition};
use crate::days::day02::ReportStatus::{Safe, Unsafe};
use crate::days::{DayError, DaySolver};
use std::cmp::PartialEq;
use std::num::ParseIntError;
use thiserror::Error;

pub struct Day2Solver;

#[derive(Error, Debug)]
enum ReportError {
    #[error("Invalid level input {0}")]
    LevelInputError(String),

    #[error("Unsafe transition between levels {0} - {1}")]
    UnsafeLevelTransition(i32, i32),
}

impl From<ReportError> for DayError {
    fn from(value: ReportError) -> Self {
        match value {
            LevelInputError(err) => DayError::InvalidInputError(err),
            err => DayError::Unknown(err.to_string()),
        }
    }
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
    status: ReportStatus,
}

fn get_direction_for_levels(level1: &i32, level2: &i32) -> LevelDirection {
    match level1.cmp(level2) {
        std::cmp::Ordering::Greater => Decreasing,
        std::cmp::Ordering::Less => Increasing,
        std::cmp::Ordering::Equal => Stale,
    }
}

fn is_safe_difference(level1: &i32, level2: &i32) -> bool {
    (level1 - level2).abs() <= 3
}

fn get_report_status(levels: &Vec<i32>) -> Result<ReportStatus, ReportError> {
    if levels.len() < 2 {
        return Err(LevelInputError(
            "Levels must contain at least two entries".to_string(),
        ));
    }

    let initial_direction = get_direction_for_levels(&levels[0], &levels[1]);

    if initial_direction == Stale {
        return Ok(Unsafe);
    }

    levels
        .windows(2)
        .try_fold(initial_direction, |prev_direction, window| {
            let current_direction = get_direction_for_levels(&window[0], &window[1]);

            if current_direction != prev_direction || !is_safe_difference(&window[0], &window[1]) {
                return Err(UnsafeLevelTransition(window[0].clone(), window[1].clone()));
            }

            Ok(current_direction)
        })
        .map(|_| Safe)
        .or(Ok(Unsafe))
}

fn to_report(levels: &Vec<i32>) -> Result<Report, ReportError> {
    Ok(Report {
        status: get_report_status(levels)?,
    })
}

fn parse_input(input: &str) -> Result<Vec<Vec<i32>>, ReportError> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|item| {
                    item.parse()
                        .map_err(|e: ParseIntError| LevelInputError(e.to_string()))
                })
                .collect::<Result<Vec<i32>, _>>()
        })
        .collect()
}

impl DaySolver for Day2Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let reports = parse_input(input)
            .map_err(Into::<DayError>::into)?
            .iter()
            .map(|levels| -> Result<Report, ReportError> { Ok(to_report(levels)?) })
            .collect::<Result<Vec<Report>, ReportError>>()
            .map_err(Into::<DayError>::into)?
            .into_iter()
            .filter(|report| report.status == Safe);

        Ok(reports.count().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let reports = parse_input(input)
            .map_err(Into::<DayError>::into)?
            .into_iter()
            .filter_map(|levels| {
                // Attempt to find a safe subset by removing one element
                levels.iter().enumerate().find_map(|(idx, _)| {
                    let mut test_levels = levels.clone();
                    test_levels.remove(idx);
                    get_report_status(&test_levels)
                        .ok()
                        .filter(|status| status == &Safe)
                })
            })
            .count();

        Ok(reports.to_string())
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
            parse_input(get_example_input()).unwrap(),
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
