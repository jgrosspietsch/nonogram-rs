use ndarray::Array1;
use std::collections::{HashSet, VecDeque};

#[path = "state_grid.rs"]
mod state_grid;
#[path = "state_row.rs"]
mod state_row;
#[path = "lookup.rs"]
mod lookup;

pub use state_grid::StateGrid;
pub use state_row::StateRow;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum CellState {
    Unknown,
    Empty,
    Filled,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Unknown
    }
}

pub fn enumerate_row_states(size: usize, clue: &[usize]) -> Vec<StateRow> {
    let mut queue: VecDeque<StateRow> = VecDeque::new();
    let mut known_rows: HashSet<StateRow> = HashSet::new();

    queue.push_back(StateRow(Array1::default(size)));

    while let Some(row) = queue.pop_front() {
        if row.is_known() {
            known_rows.insert(row);
        } else {
            if let Some(seg_row) = row.new_w_appended_seg(clue) {
                queue.push_front(seg_row);
            }

            if let Some(zero_row) = row.new_w_appended_zero(clue) {
                queue.push_front(zero_row);
            }
        }
    }

    known_rows.iter().cloned().collect()
}

pub fn filter_invalid_row_states(
    known_row: &StateRow,
    previous_states: &[StateRow],
) -> Vec<StateRow> {
    previous_states
        .iter()
        .cloned()
        .filter(|state_row| {
            known_row
                .0
                .iter()
                .zip(state_row.0.iter())
                .all(|(known_cell, state_cell)| {
                    *known_cell == CellState::Unknown
                        || (*known_cell != CellState::Unknown && known_cell == state_cell)
                })
        })
        .collect()
}

pub fn common_row_indexes(states: &[StateRow]) -> Vec<(usize, CellState)> {
    let mut common_empty: Array1<usize> = Array1::zeros(states[0].0.len());
    let mut common_filled: Array1<usize> = Array1::zeros(states[0].0.len());

    for state in states {
        for i in 0..state.0.len() {
            match state.0[i] {
                CellState::Empty => common_empty[i] += 1,
                CellState::Filled => common_filled[i] += 1,
                _ => (),
            };
        }
    }

    (0..states[0].0.len())
        .filter_map(|index| {
            if common_empty[index] == states.len() {
                Some((index, CellState::Empty))
            } else if common_filled[index] == states.len() {
                Some((index, CellState::Filled))
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        common_row_indexes, enumerate_row_states, filter_invalid_row_states, CellState, StateRow,
    };
    use ndarray::arr1;

    #[test]
    fn enumerate_row_states1() {
        let states = enumerate_row_states(5, &[1, 1, 1]);

        assert_eq!(states.len(), 1);
    }

    #[test]
    fn enumerate_row_states2() {
        let states = enumerate_row_states(5, &[3, 1]);

        assert_eq!(states.len(), 1);
    }

    #[test]
    fn enumerate_row_states3() {
        let states = enumerate_row_states(5, &[1, 2]);

        assert_eq!(states.len(), 3);
    }

    #[test]
    fn enumerate_row_states4() {
        let states = enumerate_row_states(5, &[1, 1]);

        assert_eq!(states.len(), 6);
    }

    #[test]
    fn enumerate_row_states5() {
        let states = enumerate_row_states(5, &[1]);

        assert_eq!(states.len(), 5);
    }

    #[test]
    fn filter_invalid_row_states1() {
        let known = StateRow(arr1(&[
            CellState::Filled,
            CellState::Empty,
            CellState::Unknown,
            CellState::Empty,
            CellState::Unknown,
        ]));
        let enumerated_states = enumerate_row_states(5, &[1, 1]);
        let filtered_states = filter_invalid_row_states(&known, &enumerated_states);

        assert_eq!(filtered_states.len(), 2);
    }

    #[test]
    fn filter_invalid_row_states2() {
        let known = StateRow(arr1(&[
            CellState::Empty,
            CellState::Unknown,
            CellState::Filled,
            CellState::Unknown,
            CellState::Empty,
        ]));
        let enumerated_states = enumerate_row_states(5, &[2]);
        let filtered_states = filter_invalid_row_states(&known, &enumerated_states);

        assert_eq!(filtered_states.len(), 2);
    }

    #[test]
    fn filter_invalid_row_states3() {
        let known = StateRow(arr1(&[
            CellState::Filled,
            CellState::Unknown,
            CellState::Filled,
            CellState::Unknown,
            CellState::Empty,
        ]));
        let enumerated_states = enumerate_row_states(5, &[3]);
        let filtered_states = filter_invalid_row_states(&known, &enumerated_states);

        assert_eq!(filtered_states.len(), 1);
    }

    #[test]
    fn common_row_indexes1() {
        let enumerated_row_states = enumerate_row_states(5, &[3]);
        let common_cells = common_row_indexes(&enumerated_row_states);

        assert_eq!(common_cells.len(), 1);
        assert_eq!(common_cells[0].0, 2);
        assert_eq!(common_cells[0].1, CellState::Filled);
    }

    #[test]
    fn common_row_indexes2() {
        let enumerated_row_states = enumerate_row_states(5, &[2, 1]);
        let common_cells = common_row_indexes(&enumerated_row_states);

        assert_eq!(common_cells.len(), 1);
        assert_eq!(common_cells[0].0, 1);
        assert_eq!(common_cells[0].1, CellState::Filled);
    }

    #[test]
    fn common_row_indexes3() {
        let enumerated_row_states = enumerate_row_states(5, &[1, 3]);
        let common_cells = common_row_indexes(&enumerated_row_states);

        assert_eq!(common_cells.len(), 5);
        assert_eq!(common_cells[0].0, 0);
        assert_eq!(common_cells[0].1, CellState::Filled);
        assert_eq!(common_cells[1].0, 1);
        assert_eq!(common_cells[1].1, CellState::Empty);
        assert_eq!(common_cells[2].0, 2);
        assert_eq!(common_cells[2].1, CellState::Filled);
        assert_eq!(common_cells[3].0, 3);
        assert_eq!(common_cells[3].1, CellState::Filled);
        assert_eq!(common_cells[4].0, 4);
        assert_eq!(common_cells[4].1, CellState::Filled);
    }
}
