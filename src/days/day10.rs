use crate::days::{DayError, DaySolver};
use crate::utils::direction::Direction4Way;
use crate::utils::grid::Grid;
use crate::utils::point::Point;
use std::str::FromStr;
use itertools::Itertools;

pub struct Day10Solver;

pub struct HeightMapNode(u32);

type HeightMap = Grid<HeightMapNode>;

fn parse_input(input: &str) -> (HeightMap, Vec<Point>) {
    let mut starting_positions = vec![];

    (
        Grid::make(
            input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, char)| {
                            let digit = char
                                .to_digit(10)
                                .ok_or(DayError::InvalidInputError(String::from("Invalid digit")))
                                .unwrap();
                            if digit == 0 {
                                starting_positions.push(Point {
                                    x: x.try_into().unwrap(),
                                    y: y.try_into().unwrap(),
                                });
                            }
                            HeightMapNode(digit)
                        })
                        .collect::<Vec<HeightMapNode>>()
                })
                .collect::<Vec<Vec<HeightMapNode>>>(),
        ),
        starting_positions,
    )
}

fn get_trails_with_reachable_end(position: &Point, height_map: &HeightMap, current_height: u32) -> Vec<Point> {
    if (current_height == 9) {
        return return vec![position.clone()];
    }

    let next_valid_height = current_height + 1;

    Direction4Way::all_directions_turning_right(Direction4Way::Right)
        .iter()
        .flat_map(|direction| {
            let point_in_direction: Point = direction.into();
            let next_position = position + &point_in_direction;

            match height_map.get_point(&next_position) {
                Some(HeightMapNode(height)) => {
                    if height == &next_valid_height {
                        get_trails_with_reachable_end(&next_position, height_map, next_valid_height)
                    } else {
                        vec![]
                    }
                }
                None => vec![],
            }
        }).collect::<Vec<_>>()
}

impl DaySolver for Day10Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let (height_map, starting_positions) = parse_input(input);

        let trailhead_scores: usize = starting_positions
            .iter()
            .map(|position| {
                let score = get_trails_with_reachable_end(position, &height_map, 0).iter().unique().count();
                return score;
            })
            .sum();

        Ok(trailhead_scores.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let (height_map, starting_positions) = parse_input(input);

        let trailhead_scores: usize = starting_positions
            .iter()
            .map(|position| {
                let score = get_trails_with_reachable_end(position, &height_map, 0).iter().count();
                return score;
            })
            .sum();

        Ok(trailhead_scores.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day10::Day10Solver;
    use crate::days::DaySolver;

    fn get_example_input() -> &'static str {
        "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
    }

    #[test]
    fn test_part1() {
        let solution = Day10Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "36")
    }

    fn test_part2() {
        let solution = Day10Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "81")
    }
}
