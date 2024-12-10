#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Direction4Way {
    Up,
    Down,
    Left,
    Right,
}

impl Direction4Way {
    pub fn all_directions_turning_right(starting_from: Direction4Way) -> Vec<Direction4Way> {
        let mut directions = Vec::new();
        let mut current_direction = starting_from.clone();

        loop {
            directions.push(current_direction.clone());
            current_direction = current_direction.turn_90_degrees_to_right();

            if current_direction == starting_from {
                break;
            }
        }

        directions
    }

    pub fn turn_90_degrees_to_right(&self) -> Direction4Way {
        match self {
            Direction4Way::Up => Direction4Way::Right,
            Direction4Way::Down => Direction4Way::Left,
            Direction4Way::Left => Direction4Way::Up,
            Direction4Way::Right => Direction4Way::Down,
        }
    }

    pub fn opposite(&self) -> Direction4Way {
        match self {
            Direction4Way::Up => Direction4Way::Down,
            Direction4Way::Down => Direction4Way::Up,
            Direction4Way::Left => Direction4Way::Right,
            Direction4Way::Right => Direction4Way::Left
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_directions() {
        assert_eq!(
            Direction4Way::all_directions_turning_right(Direction4Way::Right),
            vec![
                Direction4Way::Right,
                Direction4Way::Down,
                Direction4Way::Left,
                Direction4Way::Up
            ]
        )
    }
}
