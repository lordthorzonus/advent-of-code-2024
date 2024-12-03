use crate::days::day03::InstructionStatus::{Disabled, Enabled};
use crate::days::{DayError, DaySolver};
use regex::Regex;
use std::sync::LazyLock;

pub struct Day3Solver;

#[derive(Debug, PartialEq)]
struct MultiplicationInstruction(i32, i32);

#[derive(PartialEq, Debug)]
enum InstructionStatus {
    Enabled,
    Disabled,
}

impl MultiplicationInstruction {
    pub fn execute(&self) -> i32 {
        self.0 * self.1
    }
}

static MULTIPLICATION_INSTRUCTION_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((?<left_number>\d*),(?<right_number>\d*)\)").unwrap());
static PART2_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap());

fn parse_line_to_multiplication_instructions<'a>(
    input: &'a str,
) -> (impl Iterator<Item = MultiplicationInstruction> + 'a) {
    MULTIPLICATION_INSTRUCTION_REGEX
        .captures_iter(input)
        .map(|caps| {
            let (_, [left_number, right_number]) = caps.extract();
            MultiplicationInstruction(left_number.parse().unwrap(), right_number.parse().unwrap())
        })
}

fn part2_parse_line_to_multiplication_instructions<'a>(
    input: &'a str,
) -> (impl Iterator<Item = MultiplicationInstruction> + 'a) {
    let mut program_status = Enabled;
    PART2_REGEX.captures_iter(input).filter_map(move |caps| {
        if let Some(line) = caps.get(0).map(|m| m.as_str()) {
            return match line {
                "do()" => {
                    program_status = Enabled;
                    None
                }
                "don't()" => {
                    program_status = Disabled;
                    None
                }
                _ => match program_status {
                    Enabled => {
                        if let (Some(left_number), Some(right_number)) = (caps.get(1), caps.get(2))
                        {
                            return Some(MultiplicationInstruction(
                                left_number.as_str().parse().unwrap(),
                                right_number.as_str().parse().unwrap(),
                            ));
                        }

                        None
                    }
                    Disabled => None,
                },
            };
        }
        None
    })
}

impl DaySolver for Day3Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let result: i32 = parse_line_to_multiplication_instructions(input)
            .map(|instruction| instruction.execute())
            .sum();
        Ok(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let result: i32 = part2_parse_line_to_multiplication_instructions(input)
            .map(|instruction| instruction.execute())
            .sum();
        Ok(result.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input1() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn get_example_input2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[test]
    fn part1() {
        let solution = Day3Solver {}.solve_part1(get_example_input1()).unwrap();
        assert_eq!(solution, "161")
    }

    #[test]
    fn part2() {
        let solution = Day3Solver {}.solve_part2(get_example_input2()).unwrap();
        assert_eq!(solution, "48")
    }
}
