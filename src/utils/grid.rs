use crate::utils::point::Point;

pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub height: u64,
    pub width: u64,
}

impl<T> Grid<T> {
    pub fn make(grid: Vec<Vec<T>>) -> Grid<T> {
        let height = grid.len();
        let width = grid[0].len();

        Grid {
            grid,
            height: height as u64,
            width: width as u64,
        }
    }

    pub fn get_point(&self, point: &Point) -> Option<&T> {
        let row: &Vec<T> = self.grid.get::<usize>(point.y.try_into().ok()?)?;

        row.get::<usize>(point.x.try_into().ok()?)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::grid::Grid;
    use crate::utils::point::Point;

    #[test]
    fn test_getting_with_point() {
        let grid = Grid::make(vec![vec!['a']]);
        let point = Point { x: 0, y: 0 };
        let non_existing_point = Point { x: 1, y: 1 };
        let invalid_index_point = Point { y: -1, x: -1 };

        let point_in_grid = grid.get_point(&point);
        assert_eq!(point_in_grid, Some(&'a'));

        let non_existing_point_in_grid = grid.get_point(&non_existing_point);
        assert_eq!(non_existing_point_in_grid, None);

        let invalid_index_point_in_grid = grid.get_point(&invalid_index_point);
        assert_eq!(invalid_index_point_in_grid, None)
    }
}
