use crate::days::{DayError, DaySolver};
use std::collections::HashSet;
use std::iter::Map;
use std::ops::Add;

pub struct Day6Solver;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
struct Coordinate(i64, i64);

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right_90_degrees(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Coordinate(0, -1),
            Direction::Down => Coordinate(0, 1),
            Direction::Left => Coordinate(-1, 0),
            Direction::Right => Coordinate(1, 0),
        }
    }
}

fn get_from_grid<'a, 'b>(
    coordinate: &'a Coordinate,
    grid: &'b Vec<Vec<MapNode>>,
) -> Option<&'a MapNode>
where
    'b: 'a,
{
    let row: &Vec<MapNode> = grid.get::<usize>(coordinate.1.try_into().ok()?)?;
    row.get::<usize>(coordinate.0.try_into().ok()?)
}

fn insert_obstacle_into_grid(obstacle_position: &Coordinate, grid: &Vec<Vec<MapNode>>) -> Vec<Vec<MapNode>> {
    let mut new_grid = grid.clone();
    let mut row: &mut Vec<MapNode> = new_grid.get_mut::<usize>(obstacle_position.1.try_into().unwrap()).unwrap();
    let x: usize = obstacle_position.0.try_into().unwrap();
    row[x] = MapNode::Obstacle;

    new_grid
}

#[derive(Debug, Clone)]
struct Guard {
    current_position: Coordinate,
    traversed_path: HashSet<Coordinate>,
    facing_direction: Direction,
}

impl Guard {
    pub fn initialize(position: Coordinate) -> Guard {
        Guard {
            current_position: position,
            traversed_path: HashSet::new(),
            facing_direction: Direction::Up,
        }
    }

    pub fn traverse_grid(&mut self, grid: &Vec<Vec<MapNode>>) -> HashSet<Coordinate> {
        let mut position: Option<Coordinate> = Some(self.current_position.clone());

        while let Some(guard_position) = &position {
            let coordinate = guard_position.clone() + self.facing_direction.clone().into();
            let next_position = get_from_grid(&coordinate, grid);

            match next_position {
                None => {
                    position = None;
                }
                Some(node) => match node {
                    MapNode::Path => {
                        self.traversed_path.insert(coordinate.clone());
                        self.current_position = coordinate.clone();
                        position = Some(coordinate.clone());
                    }
                    MapNode::Obstacle => {
                        self.facing_direction =
                            self.facing_direction.clone().turn_right_90_degrees()
                    }
                    MapNode::GuardNode => {}
                },
            }
        }

        self.traversed_path.clone()
    }

    pub fn will_be_stuck_in_loop(mut self, grid: &Vec<Vec<MapNode>>) -> bool {
        let mut positions_visited_with_direction: HashSet<(Coordinate, Direction)> = HashSet::new();
        let mut position: Option<Coordinate> = Some(self.current_position.clone());
        let mut times_visited_same_position = 0;
        let mut is_loop = false;
        while let Some(guard_position) = &position {
            let coordinate = guard_position.clone() + self.facing_direction.clone().into();
            let next_position = get_from_grid(&coordinate, grid);

            match next_position {
                None => {
                    position = None;
                }
                Some(node) => match node {
                    MapNode::Path => {
                        if (positions_visited_with_direction.contains(&(coordinate.clone(), self.facing_direction.clone()))) {
                            times_visited_same_position += 1;

                            if (times_visited_same_position > 3) {
                                is_loop = true;
                                break;
                            }
                        }
                        positions_visited_with_direction.insert((coordinate.clone(), self.facing_direction.clone()));
                        self.current_position = coordinate.clone();
                        position = Some(coordinate.clone());
                    }
                    MapNode::Obstacle => {
                        positions_visited_with_direction.insert((coordinate.clone(), self.facing_direction.clone()));
                        self.facing_direction =
                            self.facing_direction.clone().turn_right_90_degrees()
                    }
                    MapNode::GuardNode => {}
                },
            }
        }

        is_loop
    }
}

#[derive(Debug, PartialEq, Clone)]
enum MapNode {
    Path,
    Obstacle,
    GuardNode,
}

impl TryFrom<char> for MapNode {
    type Error = DayError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(MapNode::Obstacle),
            '.' => Ok(MapNode::Path),
            '^' => Ok(MapNode::GuardNode),
            c => Err(DayError::InvalidInputError(format!(
                "Unknown character encountered in the map: '{}'",
                c
            ))),
        }
    }
}

fn parse_input(input: &str) -> Result<(Vec<Vec<MapNode>>, Guard), DayError> {
    let mut initial_guard_position: Option<Coordinate> = None;
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    let node: MapNode = char.try_into()?;
                    if node == MapNode::GuardNode {
                        initial_guard_position = Some(Coordinate(
                            x.try_into().map_err(|_| {
                                DayError::InvalidInputError(String::from(
                                    "Cannot convert usize to i64",
                                ))
                            })?,
                            y.try_into().map_err(|e| {
                                DayError::InvalidInputError(String::from(
                                    "Cannot convert usize to i64",
                                ))
                            })?,
                        ));
                        return Ok(MapNode::Path);
                    }

                    Ok(node)
                })
                .collect::<Result<Vec<MapNode>, DayError>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>()?;

    match initial_guard_position {
        Some(position) => Ok((map, Guard::initialize(position))),
        None => Err(DayError::InvalidInputError(
            "Cannot parse the initial guard position from the map".to_string(),
        )),
    }
}

impl DaySolver for Day6Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let (grid, mut guard) = parse_input(input)?;
        guard.traverse_grid(&grid);

        Ok(guard.traversed_path.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let (grid, guard) = parse_input(input)?;

        let mut initial_guard = guard.clone();
        initial_guard.traverse_grid(&grid);
        let mut obstacle_coordinates_that_cause_infinite_loop: HashSet<Coordinate> = HashSet::new();

        for coordinate in initial_guard.traversed_path.iter().filter(|coordinate| *coordinate != &guard.clone().current_position) {
            let grid_with_obstacle = insert_obstacle_into_grid(&coordinate, &grid);
            let test_guard = guard.clone();

            if test_guard.will_be_stuck_in_loop(&grid_with_obstacle) {
                obstacle_coordinates_that_cause_infinite_loop.insert(coordinate.clone());
            }
        }

        Ok(obstacle_coordinates_that_cause_infinite_loop.len().to_string())
    }
}
#[cfg(test)]
mod tests {
    use crate::days::day06::Day6Solver;
    use crate::days::DaySolver;

    fn get_example_input() -> &'static str {
        "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
    }

    #[test]
    fn test_part1() {
        let solution = Day6Solver {}.solve_part1(get_example_input()).unwrap();

        assert_eq!(solution, "41")
    }

    #[test]
    fn test_part2() {
        let solution = Day6Solver {}.solve_part2(get_example_input()).unwrap();

        assert_eq!(solution, "6")
    }
}
