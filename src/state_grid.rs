extern crate ndarray;

use super::{CellState, StateRow};
use ndarray::Array2;

pub struct StateGrid(Array2<CellState>);

impl StateGrid {
    pub fn new(height: usize, width: usize) -> StateGrid {
        StateGrid(Array2::default((height, width)))
    }

    pub fn set(&mut self, i: usize, j: usize, state: CellState) {
        if self.get(i, j).is_some() {
            self.0[[i, j]] = state;
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&CellState> {
        self.0.get((i, j))
    }

    pub fn get_row(&self, i: usize) -> StateRow {
        StateRow(self.0.row(i).into_iter().cloned().collect())
    }

    pub fn get_column(&self, j: usize) -> StateRow {
        StateRow(self.0.column(j).into_iter().cloned().collect())
    }

    pub fn is_known(&self) -> bool {
        self.0.iter().all(|&cell| cell != CellState::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::{CellState, StateGrid, StateRow};
    use ndarray::arr1;

    #[test]
    fn new_grid_with_dimensions() {
        let grid = StateGrid::new(10, 5);

        assert_eq!(grid.0.dim(), (10, 5));
    }

    #[test]
    fn new_grid_with_default() {
        let grid = StateGrid::new(5, 5);

        assert_eq!(*grid.0.get((1, 1)).unwrap(), CellState::Unknown);
        assert_eq!(*grid.0.get((0, 2)).unwrap(), CellState::Unknown);
        assert_eq!(*grid.0.get((3, 4)).unwrap(), CellState::Unknown);
    }

    #[test]
    fn set_state_in_bounds() {
        let mut grid = StateGrid::new(5, 5);

        grid.set(2, 3, CellState::Filled);
        grid.set(4, 0, CellState::Empty);

        assert_eq!(*grid.0.get((2, 3)).unwrap(), CellState::Filled);
        assert_eq!(*grid.0.get((4, 0)).unwrap(), CellState::Empty);
    }

    #[test]
    fn set_state_out_of_bounds() {
        let mut grid = StateGrid::new(5, 5);

        grid.set(12, 7, CellState::Filled);
    }

    #[test]
    fn get_state_in_bounds() {
        let mut grid = StateGrid::new(5, 5);

        grid.0[[2, 3]] = CellState::Filled;
        grid.0[[4, 0]] = CellState::Empty;

        assert_eq!(*grid.get(2, 3).unwrap(), CellState::Filled);
        assert_eq!(*grid.get(4, 0).unwrap(), CellState::Empty);
    }

    #[test]
    fn get_state_out_of_bounds() {
        let grid = StateGrid::new(5, 5);

        assert!(grid.get(12, 7).is_none());
    }

    #[test]
    fn get_column_at_j() {
        let mut grid = StateGrid::new(5, 5);

        grid.set(2, 2, CellState::Filled);
        grid.set(4, 2, CellState::Empty);

        assert_eq!(
            grid.get_column(2),
            StateRow(arr1(&[
                CellState::Unknown,
                CellState::Unknown,
                CellState::Filled,
                CellState::Unknown,
                CellState::Empty
            ]))
        );
    }

    #[test]
    fn get_row_at_i() {
        let mut grid = StateGrid::new(5, 5);

        grid.set(3, 1, CellState::Filled);
        grid.set(3, 3, CellState::Empty);

        assert_eq!(
            grid.get_row(3),
            StateRow(arr1(&[
                CellState::Unknown,
                CellState::Filled,
                CellState::Unknown,
                CellState::Empty,
                CellState::Unknown
            ]))
        );
    }

    #[test]
    fn grid_is_known_false1() {
        let grid = StateGrid::new(2, 2);

        assert!(!grid.is_known());
    }

    #[test]
    fn grid_is_known_false2() {
        let mut grid = StateGrid::new(2, 2);

        grid.set(0, 0, CellState::Empty);
        grid.set(1, 1, CellState::Filled);

        assert!(!grid.is_known());
    }

    #[test]
    fn grid_is_known_true() {
        let mut grid = StateGrid::new(2, 2);

        grid.set(0, 0, CellState::Filled);
        grid.set(0, 1, CellState::Empty);
        grid.set(1, 0, CellState::Filled);
        grid.set(1, 1, CellState::Empty);

        assert!(grid.is_known());
    }
}
