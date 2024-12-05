use crate::days::{DayError, DaySolver};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::num::ParseIntError;

pub struct Day5Solver;

#[derive(PartialEq, Debug)]
pub struct Rule {
    pub number_before: u32,
    pub number_after: u32,
}

fn rules_to_sort_lookup_map(rules: &Vec<Rule>) -> HashMap<u32, Vec<u32>> {
    let mut lookup: HashMap<u32, Vec<u32>> = HashMap::new();

    for rule in rules {
        let numbers_after = lookup.entry(rule.number_before).or_insert(vec![]);
        numbers_after.push(rule.number_after);
    }

    lookup
}

impl std::str::FromStr for Rule {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((number_before_str, number_after_str)) = s.split_once("|") {
            return Ok(Rule {
                number_before: number_before_str
                    .parse()
                    .map_err(|e: ParseIntError| DayError::InvalidInputError(e.to_string()))?,
                number_after: number_after_str
                    .parse()
                    .map_err(|e: ParseIntError| DayError::InvalidInputError(e.to_string()))?,
            });
        }

        Err(DayError::InvalidInputError(
            "Cannot split the rule line with |".to_string(),
        ))
    }
}

fn parse_input(input: &str) -> Result<(Vec<Rule>, Vec<Vec<u32>>), DayError> {
    if let Some((rule_str, updates_str)) = input.split_once("\n\n") {
        let rules: Vec<Rule> = rule_str
            .lines()
            .map(|rule_line| rule_line.parse())
            .collect::<Result<Vec<Rule>, DayError>>()?;
        let updates: Vec<Vec<u32>> = updates_str
            .lines()
            .map(|update_line| {
                update_line
                    .split(",")
                    .filter_map(|c| c.parse().ok())
                    .collect::<Vec<u32>>()
            })
            .collect();

        return Ok((rules, updates));
    }

    Err(DayError::InvalidInputError(
        "Invalid input, cannot split into rule and update lists".to_string(),
    ))
}

fn sort_by_rule_lookup(
    lookup_map: &HashMap<u32, Vec<u32>>,
    number_a: &u32,
    number_b: &u32,
) -> bool {
    if let Some(numbers_after) = lookup_map.get(number_a) {
        if numbers_after.contains(number_b) {
            return true;
        }

        return false;
    }

    false
}

impl DaySolver for Day5Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let (rules, updates) = parse_input(input)?;
        let lookup_map = rules_to_sort_lookup_map(&rules);

        let valid_updates = updates
            .iter()
            .filter(|update| {
                update.is_sorted_by(|number_a, number_b| {
                    sort_by_rule_lookup(&lookup_map, number_a, number_b)
                })
            })
            .collect::<Vec<_>>();

        let sum_of_middle_numbers: u32 = valid_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum();

        Ok(sum_of_middle_numbers.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let (rules, mut updates) = parse_input(input)?;
        let lookup_map = rules_to_sort_lookup_map(&rules);

        let sorted_invalid_updates = updates
            .iter_mut()
            .filter(|update| {
                !update.is_sorted_by(|number_a, number_b| {
                    sort_by_rule_lookup(&lookup_map, number_a, number_b)
                })
            })
            .map(|mut update| {
                update.sort_by(|number_a, number_b| {
                    if sort_by_rule_lookup(&lookup_map, number_a, number_b) {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    }
                });

                update
            })
            .collect::<Vec<_>>();

        let sum_of_middle_numbers: u32 = sorted_invalid_updates
            .iter()
            .map(|update| update[update.len() / 2])
            .sum();

        Ok(sum_of_middle_numbers.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }

    #[test]
    fn test_input_parsing() {
        let (rules, updates) = parse_input(
            "\
47|53
97|13

75,29,13
61,13,29",
        )
        .unwrap();

        assert_eq!(
            vec![
                Rule {
                    number_before: 47,
                    number_after: 53
                },
                Rule {
                    number_before: 97,
                    number_after: 13
                }
            ],
            rules
        );
        assert_eq!(vec![vec![75, 29, 13], vec![61, 13, 29]], updates);
    }

    #[test]
    fn test_part1() {
        let solution = Day5Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "143")
    }

    #[test]
    fn test_part2() {
        let solution = Day5Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "123")
    }
}
