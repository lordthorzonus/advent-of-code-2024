use crate::utils::direction::Direction4Way;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<&Direction4Way> for Point {
    fn from(value: &Direction4Way) -> Self {
        match value {
            Direction4Way::Up => Point { x: 0, y: 1 },
            Direction4Way::Down => Point { x: 0, y: -1 },
            Direction4Way::Left => Point { x: -1, y: 0 },
            Direction4Way::Right => Point { x: 1, y: 0 },
        }
    }
}
