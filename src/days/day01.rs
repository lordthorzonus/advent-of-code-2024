use crate::days::{DayError, DaySolver};
use std::collections::HashMap;
use std::num::ParseIntError;

pub struct Day1Solver;

fn parse_input_to_lists(input: &str) -> Result<(Vec<i32>, Vec<i32>), DayError> {
    Ok(input
        .lines()
        .filter_map(|line| line.split_once("   "))
        .map(|numbers_as_string| -> Result<(i32, i32), ParseIntError> {
            Ok((numbers_as_string.0.parse()?, numbers_as_string.1.parse()?))
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            DayError::InvalidInputError(
                "Cannot parse the given input into number list".to_string(),
            )
        })?
        .into_iter()
        .unzip())
}

fn count_occurrences(list: &Vec<i32>) -> HashMap<i32, i32> {
    list.iter().fold(HashMap::new(), |mut map, &list_entry| {
        map.entry(list_entry)
            .and_modify(|frequency| *frequency += 1)
            .or_insert(1);
        map
    })
}

fn calculate_similarity(number: &i32, occurrences: &HashMap<i32, i32>) -> i32 {
    let number_of_occurrences = occurrences.get(number).unwrap_or(&0);

    number * number_of_occurrences
}

impl DaySolver for Day1Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let (mut left_list, mut right_list) = parse_input_to_lists(input)?;

        if left_list.len() != right_list.len() {
            return Err(DayError::InvalidInputError(
                "The given lists are not of the same length.".to_string(),
            ));
        }

        left_list.sort();
        right_list.sort();

        let distances: i32 = left_list
            .iter()
            .zip(right_list.iter())
            .map(|(left, right)| (left - right).abs())
            .sum();

        Ok(distances.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let (left_list, right_list) = parse_input_to_lists(input)?;
        let occurrences = count_occurrences(&right_list);
        let similarities: i32 = left_list
            .iter()
            .map(|entry| calculate_similarity(entry, &occurrences))
            .sum();

        Ok(similarities.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input()  -> &'static str{
        "3   4
4   3
2   5
1   3
3   9
3   3\
        "
    }
    #[test]
    fn part1() {
        let solution = Day1Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "11")
    }

    #[test]
    fn part2() {
        let solution = Day1Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "31")
    }
}
