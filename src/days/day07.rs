use crate::days::{DayError, DaySolver};

pub struct Day7Solver;

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

impl Operator {
    pub fn execute(&self, a: i64, b: i64) -> i64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concat => format!("{a}{b}").parse().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Equation {
    pub fn is_solvable(&self, allowed_operators: &Vec<Operator>) -> bool {
        Self::has_solution(allowed_operators, &self.numbers, 0, self.target)
    }

    fn has_solution(operators: &Vec<Operator>, numbers: &[i64], current: i64, target: i64) -> bool {
        if current > target {
            return false;
        }

        if numbers.len() == 0 {
            return current == target;
        }

        let next_number = numbers[0];
        operators.iter().any(|operator| {
            Self::has_solution(
                operators,
                &numbers[1..],
                operator.execute(current, next_number),
                target,
            )
        })
    }
}

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (target_str, numbers_str) = line
                .split_once(":")
                .ok_or(DayError::InvalidInputError(
                    "Could not parse Equation input".to_string(),
                ))
                .unwrap();
            Equation {
                target: target_str.parse().unwrap(),
                numbers: numbers_str
                    .trim()
                    .split(" ")
                    .map(|number| number.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

impl DaySolver for Day7Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let valid_equations: Vec<Equation> = parse_input(input)
            .into_iter()
            .filter(|equation| equation.is_solvable(&vec![Operator::Add, Operator::Multiply]))
            .collect();

        let sum_of_equations: i64 = valid_equations.iter().map(|equation| equation.target).sum();

        Ok(sum_of_equations.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let valid_equations: Vec<Equation> = parse_input(input)
            .into_iter()
            .filter(|equation| {
                equation.is_solvable(&vec![Operator::Add, Operator::Multiply, Operator::Concat])
            })
            .collect();

        let sum_of_equations: i64 = valid_equations.iter().map(|equation| equation.target).sum();

        Ok(sum_of_equations.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day07::{parse_input, Day7Solver, Equation};
    use crate::days::DaySolver;

    fn get_example_input() -> &'static str {
        return "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    }

    #[test]
    fn test_parsing() {
        let parsed = parse_input(
            "\
12: 10 19
13: 1 2",
        );
        assert_eq!(
            parsed,
            vec![
                Equation {
                    target: 12,
                    numbers: vec![10, 19]
                },
                Equation {
                    target: 13,
                    numbers: vec![1, 2]
                }
            ]
        )
    }

    #[test]
    fn test_part1() {
        let solution = Day7Solver {}.solve_part1(get_example_input()).unwrap();

        assert_eq!(solution, "3749")
    }
    #[test]
    fn test_part2() {
        let solution = Day7Solver {}.solve_part2(get_example_input()).unwrap();

        assert_eq!(solution, "11387")
    }
}
