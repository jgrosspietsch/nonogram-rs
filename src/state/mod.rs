use ndarray::Array1;
use std::collections::{HashSet, VecDeque};

mod state_grid;
mod state_row;

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

// https://math.stackexchange.com/questions/1462099/number-of-possible-combinations-of-x-numbers-that-sum-to-y
fn total_possible_rows(available: usize, segments: usize) -> usize {
    let dividend = (available + 1..=(available + segments)).product();
    let divisor = (1..=segments).product();

    dividend / divisor
}

// fn row_matches_clue(row: &[bool], clue: &[usize]) -> bool {
//     row.split(|cell| cell == true)
//         .filter_map(|seg| if seg.len() > 0 { Some(l) } else { None })
//         .eq(clue)
// }

fn space_width_to_row_segments(segments: &[usize], spaces: &[usize]) -> Vec<(usize, usize)> {
    let segment_iter = segments.into_iter();
    let spaces_iter = spaces.into_iter();
    let mut row_segs = Vec::with_capacity(segments.len());

    for (i, seg) in segment_iter.enumerate() {
        if i == 0 {
            row_segs.push((spaces[0], spaces[0] + seg));
        } else {
            row_segs.push((
                spaces[i] + row_segs[i - 1].1,
                spaces[i] + row_segs[i - 1].1 + seg,
            ));
        }
    }

    row_segs
}

pub fn enumerate_row_states(size: usize, clue: &[usize]) -> Vec<Vec<(usize, usize)>> {
    let available_spaces = size - clue.iter().sum() - (clue.len() - 1);
    let space_segments = clue.len() + 1;
    let num_possible = total_possible_rows(available_spaces, space_segments);
    let mut queue: VecDeque<Vec<usize>> = VecDeque::with_capacity(num_possible);
    let mut known_rows: HashSet<Vec<(usize, usize)>> = HashSet::with_capacity(num_possible);

    queue.push_front(Vec::<usize>::with_capacity(space_segments));

    while let Some(row_spaces) = queue.pop_front() {
        let remaining_available = row_spaces.iter().sum();

        if row_spaces.len() == space_segments - 1 {
            row_spaces.push(remaining_available);
            known_rows.insert(space_width_to_row_segments(clue, &row_spaces));
        } else {
            for seg in 0..remaining_available {
                let new_row = row_spaces.clone();
                new_row.push(seg);

                queue.push_front(new_row);
            }
        }
    }

    known_rows.iter().cloned().collect()
}

pub fn filter_invalid_row_states(
    known_row: &StateRow,
    previous_states: &[&[(usize, usize)]],
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

pub fn common_row_indexes(states: &[&[(usize, usize)]]) -> Vec<(usize, CellState)> {
    let mut common_empty: Array1<usize> = Array1::zeros(states[0].len());
    let mut common_filled: Array1<usize> = Array1::zeros(states[0].len());

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
