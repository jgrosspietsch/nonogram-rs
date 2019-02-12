extern crate ndarray;

use ndarray::{Array1, arr1};
use super::CellState;


#[derive(Eq, Clone, Debug, PartialEq, Hash)]
pub struct StateRow(pub Array1<CellState>);

impl StateRow {
    pub fn is_known(&self) -> bool {
        self.0.iter().all(|&cell| cell != CellState::Unknown)
    }

    pub fn new_w_appended_seg(&self, clue: &[usize]) -> Option<StateRow> {
        match self.next_segment(clue) {
            Some(seg) => {
                let mut appended_seg: Vec<CellState> = Vec::new();

                if !self.trailing_space() {
                    appended_seg.push(CellState::Empty);
                }

                for _ in 0..seg {
                    appended_seg.push(CellState::Filled);
                }

                Some(self.append(&appended_seg))
            },
            None => None
        }
    }

    pub fn new_w_appended_zero(&self, clue: &[usize]) -> Option<StateRow> {
        if self.remaining_cells() > self.required_cells(clue) {
            Some(self.append(&[CellState::Empty]))
        } else {
            None
        }
    }

    pub fn state_at_index(&self, index: usize) -> Option<CellState> {
        self.0.iter()
            .cloned()
            .nth(index)
    }

    fn append(&self, appended: &[CellState]) -> StateRow {
        let mut known_portion = self.0.to_vec();
        
        known_portion.truncate(match self.last_known() {
            Some(n) => n + 1,
            None => 0
        });

        for &i in appended.iter() {
            known_portion.push(i);
        }

        if self.0.len() > known_portion.len() {
            for _ in 0..self.0.len() - known_portion.len() {
                known_portion.push(CellState::Unknown);
            }
        }

        StateRow(arr1(&known_portion))
    }

    fn last_known(&self) -> Option<usize> {
        if !self.0.is_empty() && self.0[0] != CellState::Unknown {
            Some(self.0.iter().fold(0, |acc, cell| {
                match cell {
                    CellState::Unknown => acc,
                    _ => acc + 1
                }
            }) - 1)
        } else {
            None
        }
    }

    fn trailing_space(&self) -> bool {
        match self.last_known() {
            Some(n) => match self.state_at_index(n) {
                Some(state) => state == CellState::Empty,
                None => false
            },
            None => true
        }
    }

    fn remaining_cells(&self) -> usize {
        match self.last_known() {
            Some(index) => self.0.len() - (index + 1),
            None => self.0.len()
        }
    }

    fn used_segments(&self) -> Vec<usize> {
        let mut used: Vec<usize> = Vec::new();

        if let Some(row_slice) = self.0.as_slice() {
            row_slice.split(|&cell| {
                cell == CellState::Empty || cell == CellState::Unknown
            }).for_each(|segment| {
                if !segment.is_empty() {
                    used.push(segment.len());
                }
            });
        }

        used
    }

    fn remaining_segments(&self, clue: &[usize]) -> Vec<usize> {
        clue.iter()
            .skip(self.used_segments().len())
            .cloned()
            .collect()
    }

    fn next_segment(&self, clue: &[usize]) -> Option<usize> {
        self.remaining_segments(clue)
            .iter()
            .cloned()
            .nth(0)
    }

    fn required_cells(&self, clue: &[usize]) -> usize {
        let remaining_segs = self.remaining_segments(clue);

        if !remaining_segs.is_empty() {
            let segment_cells: usize = remaining_segs.iter().sum();
            let spaces_required = if !remaining_segs.is_empty() { remaining_segs.len() - 1 } else { 0 };
            let leading_space = match self.last_known() {
                Some(n) => if self.0[n] == CellState::Filled { 1 } else { 0 }
                None => 0
            };

            segment_cells + spaces_required + leading_space
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use ndarray::arr1;
    use super::{StateRow, CellState};

    #[test]
    fn row_is_known_false1() {
        let row = StateRow(arr1(&[CellState::Unknown; 5]));

        assert!(!row.is_known());
    }

    #[test]
    fn row_is_known_false2() {
        let row = StateRow(arr1(&[
            CellState::Unknown,
            CellState::Filled,
            CellState::Unknown,
            CellState::Empty,
            CellState::Unknown
        ]));

        assert!(!row.is_known());
    }

    #[test]
    fn row_is_known_true() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled
        ]));

        assert!(row.is_known());
    }

    #[test]
    fn new_w_appended_seg_no_trailing_unknown() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Unknown
        ]));

        let expected = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled
        ]));

        assert_eq!(row.new_w_appended_seg(&[3, 1]).unwrap(), expected);
    }

    #[test]
    fn new_w_appended_seg_with_trailing_unknown() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Filled,
            CellState::Unknown,
            CellState::Unknown
        ]));

        let expected = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled
        ]));

        assert_eq!(row.new_w_appended_seg(&[3, 1]).unwrap(), expected);
    }

    #[test]
    fn new_w_appended_seg_segments_exhausted() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Empty,
            CellState::Filled,
            CellState::Unknown,
            CellState::Unknown
        ]));

        assert!(row.new_w_appended_seg(&[1, 1]).is_none());
    }

    #[test]
    fn new_w_appended_zero_usable_space_remaining() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Unknown,
            CellState::Unknown,
            CellState::Unknown,
            CellState::Unknown
        ]));

        let expected = StateRow(arr1(&[
            CellState::Filled,
            CellState::Empty,
            CellState::Unknown,
            CellState::Unknown,
            CellState::Unknown
        ]));

        assert_eq!(row.new_w_appended_zero(&[1, 1]).unwrap(), expected);
    }

    #[test]
    fn new_w_appended_zero_no_usable_space() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Unknown,
            CellState::Unknown,
            CellState::Unknown,
            CellState::Unknown
        ]));

        assert!(row.new_w_appended_zero(&[1, 3]).is_none());
    }

    #[test]
    fn state_at_index_in_bounds() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled,
            CellState::Unknown
        ]));

        assert_eq!(row.state_at_index(2).unwrap(), CellState::Empty);
        assert_eq!(row.state_at_index(3).unwrap(), CellState::Filled);
        assert_eq!(row.state_at_index(4).unwrap(), CellState::Unknown);
    }

    #[test]
    fn state_at_index_out_of_bounds() {
        let row = StateRow(arr1(&[
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled,
            CellState::Unknown
        ]));

        assert!(row.state_at_index(5).is_none());
    }
}
