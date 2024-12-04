use crate::days::{DayError, DaySolver};
use std::sync::LazyLock;
use thiserror::Error;

pub struct Day4Solver;

#[derive(Error, Debug)]
pub enum XmasError {
    #[error("The XMAS maze does not yet support letter {0}")]
    NotSupportedLetterError(char),
}

#[derive(Debug, PartialEq)]
enum XmasLetter {
    X,
    M,
    A,
    S,
}

impl XmasLetter {
    pub fn get_next_expected_letter(&self) -> Option<XmasLetter> {
        match self {
            XmasLetter::X => Some(XmasLetter::M),
            XmasLetter::M => Some(XmasLetter::A),
            XmasLetter::A => Some(XmasLetter::S),
            XmasLetter::S => None,
        }
    }
}
#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn advance_to_direction(&self, direction: &Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => {
                if self.y == 0 {
                    return None;
                }

                Some(Coordinate {
                    x: self.x.clone(),
                    y: self.y.clone() - 1,
                })
            }
            Direction::Down => Some(Coordinate {
                x: self.x.clone(),
                y: self.y.clone() + 1,
            }),
            Direction::Left => {
                if self.x == 0 {
                    return None;
                }

                Some(Coordinate {
                    x: self.x.clone() - 1,
                    y: self.y.clone(),
                })
            }
            Direction::Right => Some(Coordinate {
                x: self.x.clone() + 1,
                y: self.y.clone(),
            }),
            Direction::TopLeft => {
                if self.x == 0 || self.y == 0 {
                    return None;
                }

                Some(Coordinate {
                    x: self.x.clone() - 1,
                    y: self.y.clone() - 1,
                })
            }
            Direction::TopRight => {
                if self.y == 0 {
                    return None;
                }

                Some(Coordinate {
                    x: self.x.clone() + 1,
                    y: self.y.clone() - 1,
                })
            }
            Direction::BottomLeft => {
                if self.x == 0 {
                    return None;
                }
                Some(Coordinate {
                    x: self.x.clone() - 1,
                    y: self.y.clone() + 1,
                })
            }

            Direction::BottomRight => Some(Coordinate {
                x: self.x.clone() + 1,
                y: self.y.clone() + 1,
            }),
        }
    }

    fn get_from_grid<'a, 'b>(&'a self, grid: &'b Vec<Vec<XmasLetter>>) -> Option<&'a XmasLetter>
    where
        'b: 'a,
    {
        let row = grid.get(self.y)?;
        row.get(self.x)
    }
}

impl TryFrom<char> for XmasLetter {
    type Error = XmasError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(XmasLetter::X),
            'M' => Ok(XmasLetter::M),
            'A' => Ok(XmasLetter::A),
            'S' => Ok(XmasLetter::S),
            c => Err(XmasError::NotSupportedLetterError(c)),
        }
    }
}

static LOOKUP_ORDER: LazyLock<Vec<Direction>> = LazyLock::new(|| {
    vec![
        Direction::Right,
        Direction::BottomRight,
        Direction::Down,
        Direction::BottomLeft,
        Direction::Left,
        Direction::TopLeft,
        Direction::Up,
        Direction::TopRight,
    ]
});

fn is_xmas_in_direction(
    position: &Coordinate,
    direction: &Direction,
    grid: &Vec<Vec<XmasLetter>>,
) -> bool {
    let mut current_position = *position;
    let mut next_expected_letter = Some(XmasLetter::X);

    while let (Some(current_letter), Some(expected_letter)) =
        (current_position.get_from_grid(grid), &next_expected_letter)
    {
        if current_letter == expected_letter {
            next_expected_letter = expected_letter.get_next_expected_letter();
        } else {
            break;
        }

        if let Some(next_position) = current_position.advance_to_direction(&direction) {
            current_position = next_position.clone();
        } else {
            break;
        }
    }

    if next_expected_letter.is_none() {
        return true;
    }

    false
}

fn lookup_for_xmas(
    coordinate: Coordinate,
    grid: &Vec<Vec<XmasLetter>>,
) -> Vec<(Coordinate, &Direction)> {
    LOOKUP_ORDER
        .iter()
        .filter_map(|direction| -> Option<(Coordinate, &Direction)> {
            if is_xmas_in_direction(&coordinate, &direction, grid) {
                return Some((coordinate.clone(), direction));
            }
            None
        })
        .collect::<Vec<(Coordinate, &Direction)>>()
}

fn is_xmas_cross(position: &Coordinate, grid: &Vec<Vec<XmasLetter>>) -> bool {
    if let (Some(top_left), Some(top_right), Some(bottom_right), Some(bottom_left)) = (
        position.advance_to_direction(&Direction::TopLeft),
        position.advance_to_direction(&Direction::TopRight),
        position.advance_to_direction(&Direction::BottomRight),
        position.advance_to_direction(&Direction::BottomLeft),
    ) {
        if let (
            Some(top_left_letter),
            Some(top_right_letter),
            Some(bottom_right_letter),
            Some(bottom_left_letter),
        ) = (
            top_left.get_from_grid(grid),
            top_right.get_from_grid(grid),
            bottom_right.get_from_grid(grid),
            bottom_left.get_from_grid(grid),
        ) {
            if matches!(
                (
                    top_left_letter,
                    bottom_right_letter,
                    bottom_left_letter,
                    top_right_letter
                ),
                (XmasLetter::M, XmasLetter::S, XmasLetter::M, XmasLetter::S)
                    | (XmasLetter::S, XmasLetter::M, XmasLetter::S, XmasLetter::M)
                    | (XmasLetter::M, XmasLetter::S, XmasLetter::S, XmasLetter::M)
                    | (XmasLetter::S, XmasLetter::M, XmasLetter::M, XmasLetter::S)
            ) {
                return true;
            }
        }
    }

    false
}

fn parse_input_to_grid(input: &str) -> Result<Vec<Vec<XmasLetter>>, XmasError> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| -> Result<XmasLetter, XmasError> { Ok(char.try_into()?) })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()
}

fn calculate_the_xmases_in_grid(grid: &Vec<Vec<XmasLetter>>) -> Vec<(Coordinate, &Direction)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y_index, row)| {
            row.iter()
                .enumerate()
                .flat_map(|(x_index, column)| {
                    if column == &XmasLetter::X {
                        return lookup_for_xmas(
                            Coordinate {
                                x: x_index,
                                y: y_index,
                            },
                            grid,
                        );
                    }
                    vec![]
                })
                .collect::<Vec<(Coordinate, &Direction)>>()
        })
        .collect::<Vec<_>>()
}

impl DaySolver for Day4Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let grid =
            parse_input_to_grid(input).map_err(|e| DayError::InvalidInputError(e.to_string()))?;
        let amount_of_xmas = calculate_the_xmases_in_grid(&grid);

        Ok(amount_of_xmas.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let grid =
            parse_input_to_grid(input).map_err(|e| DayError::InvalidInputError(e.to_string()))?;

        let crosses = grid
            .iter()
            .enumerate()
            .flat_map(|(y_index, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x_index, column)| {
                        let coordinate = Coordinate {
                            x: x_index,
                            y: y_index,
                        };
                        if column == &XmasLetter::A && is_xmas_cross(&coordinate, &grid) {
                            return Some(coordinate);
                        }
                        None
                    })
                    .collect::<Vec<Coordinate>>()
            })
            .collect::<Vec<_>>();

        return Ok(crosses.len().to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn get_example_input() -> &'static str {
        "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }

    #[test]
    fn test_1_xmas() {
        let solution = Day4Solver {}
            .solve_part1(
                "\
XMAS
MMMM
AMAM
SMMS
SAMX
",
            )
            .unwrap();
        assert_eq!(solution, "4")
    }

    #[test]
    fn part1() {
        let solution = Day4Solver {}.solve_part1(get_example_input()).unwrap();
        assert_eq!(solution, "18")
    }

    #[test]
    fn part2() {
        let solution = Day4Solver {}.solve_part2(get_example_input()).unwrap();
        assert_eq!(solution, "9")
    }
}
