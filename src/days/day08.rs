use crate::days::{DayError, DaySolver};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::successors;
use std::ops::{Add, Neg, Sub};

pub struct Day8Solver;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Frequency(char);

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Point {
    pub fn from_index(x: usize, y: usize) -> Point {
        Point {
            x: x.try_into().unwrap(),
            y: y.try_into().unwrap(),
        }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y
        }
    }
}

struct Antenna {
    point: Point,
}

type AntennaMap = HashMap<Frequency, Vec<Antenna>>;

struct Map {
    height: isize,
    width: isize,
    antennas: AntennaMap,
}

impl Map {
    fn is_within(&self, point: &Point) -> bool {
        (point.x >= 0 && point.x <= self.width - 1) && (point.y >= 0 && point.y <= self.height - 1)
    }

    fn cast_ray<'a,'b>(&self, start_point: &'a Point, difference: &'b Point) -> impl Iterator<Item = Point>  + use<'a, 'b, '_> {
        successors(Some(start_point.clone()), |point| {
            let next = point + difference;
            if self.is_within(&next) {
                return Some(next.clone());
            }
            None
        })
    }

    pub fn get_antinodes_part1(&self) -> HashSet<Point> {
        self.antennas
            .values()
            .flat_map(|antenna_points| {
                antenna_points
                    .iter()
                    .tuple_combinations()
                    .flat_map(|(antenna_a, antenna_b)| {
                        let difference = &antenna_a.point - &antenna_b.point;
                        [
                            &antenna_a.point + &difference,
                            &antenna_b.point - &difference,
                        ]
                        .into_iter()
                        .filter(|point| self.is_within(point))
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<HashSet<_>>()
    }

    pub fn get_antinodes_part2(&self) -> HashSet<Point> {
        self.antennas
            .values()
            .flat_map(|antenna_points| {
                antenna_points
                    .iter()
                    .tuple_combinations()
                    .flat_map(|(antenna_a, antenna_b)| {
                        let difference = &antenna_a.point - &antenna_b.point;
                        let mut antenna_a_ray = self.cast_ray(&antenna_a.point, &difference).collect::<Vec<_>>();
                        let negative_difference = -&difference;
                        let antenna_b_ray = self.cast_ray(&antenna_b.point, &negative_difference);
                        antenna_a_ray.extend(antenna_b_ray);

                        antenna_a_ray
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<HashSet<_>>()
    }
}

fn parse_input(input: &str) -> Map {
    let mut antenna_map: AntennaMap = HashMap::new();
    let lines = input.lines();
    let height = input.lines().count();
    let mut width: usize = 0;

    lines.enumerate().for_each(|(y, line)| {
        width = line.chars().count();
        line.chars().enumerate().for_each(|(x, char)| match char {
            '.' => (),
            c => {
                let point = Point::from_index(x, y);
                let frequency = Frequency(c);
                let antennas = antenna_map.entry(frequency.clone()).or_insert(vec![]);
                antennas.push(Antenna { point })
            }
        })
    });

    Map {
        width: width.try_into().unwrap(),
        height: height.try_into().unwrap(),
        antennas: antenna_map,
    }
}

impl DaySolver for Day8Solver {
    fn solve_part1(&self, input: &str) -> Result<String, DayError> {
        let map = parse_input(input);
        let antinodes = map.get_antinodes_part1();

        Ok(antinodes.len().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String, DayError> {
        let map = parse_input(input);
        let antinodes = map.get_antinodes_part2();

        Ok(antinodes.len().to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::days::day08::Day8Solver;
    use crate::days::DaySolver;

    pub fn get_test_input() -> &'static str {
        "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
    }

    #[test]
    fn test_part1() {
        let solution = Day8Solver {}.solve_part1(get_test_input()).unwrap();
        assert_eq!(solution, "14")
    }
    #[test]
    fn test_part2() {
        let solution = Day8Solver {}.solve_part2(get_test_input()).unwrap();
        assert_eq!(solution, "34")
    }
}
