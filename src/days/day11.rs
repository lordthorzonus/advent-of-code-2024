use std::collections::HashMap;
use crate::days::{DayError, DaySolver};

pub struct Day11Solver;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stone(u64);

impl Stone {
    fn is_even_digits(&self) -> bool {
        self.0.to_string().chars().count() % 2 == 0
    }

    fn split(&self) -> Vec<Stone> {
        let digit_as_string = self.0.to_string();
        let digit_length = digit_as_string.len();

        let (first_half, second_half) = digit_as_string.split_at(digit_length / 2);
        let second_half_without_leading_zeroes = second_half.trim_start_matches("0");

        let second_stone = match second_half_without_leading_zeroes {
            c if c.is_empty() => Stone(0),
            c => Stone(c.parse().unwrap()),
        };

        let first_stone = Stone(first_half.parse().unwrap());
        vec![first_stone, second_stone]
    }

    fn blink(&self) -> Vec<Stone> {
        match self {
            Stone(0) => vec![Stone(1)],
            Stone(_) if self.is_even_digits() => self.split(),
            Stone(d) => vec![Stone(d * 2024)],
        }
    }
}

fn parse_input(input: &str) -> Vec<Stone> {
    input.trim()
        .split(" ")
        .map(|number| Stone(number.parse().unwrap()))
        .collect()
}

impl DaySolver for Day11Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let mut stones = parse_input(input);

        for i in 0..25 {
            let mut new_stones = Vec::new();
            for (index, stone) in stones.iter().enumerate() {
                let changed_stones = stone.blink();
                match changed_stones.as_slice() {
                    [stone] => new_stones.push(stone.clone()),
                    [stone1, stone2] => {
                        new_stones.push(stone1.clone());
                        new_stones.push(stone2.clone())
                    }
                    &[] | &[_, _, _, ..] => todo!()
                }
            }
            stones = new_stones;
        }

        Ok(stones.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let mut stones = parse_input(input);
        let mut stone_counts: HashMap<Stone, u64> = HashMap::new();
        for stone in stones {
           *stone_counts.entry(stone).or_default() += 1;
        }

        for i in 0..75 {
            let mut new_stone_counts: HashMap<Stone, u64> = HashMap::new();
            for (stone, count) in &stone_counts{
                let changed_stones = stone.blink();
                match changed_stones.as_slice() {
                    [stone] => *new_stone_counts.entry(stone.clone()).or_default() += count,
                    [stone1, stone2] => {
                        *new_stone_counts.entry(stone1.clone()).or_default() += count;
                        *new_stone_counts.entry(stone2.clone()).or_default() += count;
                    }
                    &[] | &[_, _, _, ..] => todo!()
                }
            }
            stone_counts = new_stone_counts;
        }

        Ok(stone_counts.values().sum::<u64>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_example_input() -> &'static str {
        "125 17"
    }

    #[test]
    fn test_part1() {
        let solution = Day11Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "55312");
    }
}
