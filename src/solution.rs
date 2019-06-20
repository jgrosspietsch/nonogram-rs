use itertools::Itertools;
use num_bigint::BigUint;

mod grid {
    #[derive(PartialEq, Eq, Copy, Clone)]
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

    impl std::fmt::Display for CellState {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                CellState::Unknown => write!(f, "?"),
                CellState::Empty => write!(f, "0"),
                CellState::Filled => write!(f, "1"),
            }
        }
    }

    pub struct StateGrid {
        grid: Vec<CellState>,
        height: usize,
        width: usize,
    }

    impl StateGrid {
        pub fn new(height: usize, width: usize) -> Self {
            StateGrid {
                grid: (0..(height * width)).map(|_| CellState::Unknown).collect(),
                height,
                width,
            }
        }

        pub fn get(&self, i: usize, j: usize) -> CellState {
            self.grid[self.width * i + j]
        }

        pub fn set(&mut self, i: usize, j: usize, val: CellState) {
            self.grid[self.width * i + j] = val;
        }

        pub fn row_iter(&mut self, row_num: usize) -> RowIterator {
            RowIterator::new(self, row_num)
        }

        pub fn column_iter(&mut self, col_num: usize) -> ColumnIterator {
            ColumnIterator::new(self, col_num)
        }

        pub fn is_known(&self) -> bool {
            !self.grid.iter().any(|cell| *cell == CellState::Unknown)
        }

        // pub fn print_grid(&self) {
        //     println!("Current grid state");
        //     for (idx, cell) in self.grid.iter().enumerate() {
        //         print!("{}", cell);

        //         if (idx + 1) % self.width == 0 {
        //             print!("\n");
        //         }
        //     }
        // }
    }

    pub struct RowIterator<'a> {
        grid: &'a StateGrid,
        row_num: usize,
        counter: usize,
    }

    impl<'a> RowIterator<'a> {
        fn new(grid: &'a StateGrid, row_num: usize) -> RowIterator {
            RowIterator {
                grid,
                row_num,
                counter: 0,
            }
        }
    }

    impl<'a> Iterator for RowIterator<'a> {
        type Item = CellState;

        fn next(&mut self) -> Option<Self::Item> {
            if self.counter >= self.grid.width {
                None
            } else {
                let state = self.grid.get(self.row_num, self.counter);
                self.counter += 1;
                Some(state)
            }
        }
    }

    pub struct ColumnIterator<'a> {
        grid: &'a StateGrid,
        col_num: usize,
        counter: usize,
    }

    impl<'a> ColumnIterator<'a> {
        fn new(grid: &'a StateGrid, col_num: usize) -> ColumnIterator {
            ColumnIterator {
                grid,
                col_num,
                counter: 0,
            }
        }
    }

    impl<'a> Iterator for ColumnIterator<'a> {
        type Item = CellState;

        fn next(&mut self) -> Option<Self::Item> {
            if self.counter >= self.grid.height {
                None
            } else {
                let state = self.grid.get(self.counter, self.col_num);
                self.counter += 1;
                Some(state)
            }
        }
    }
}

struct SegmentPair {
    start: usize,
    end: usize,
}

type RowSegments = Vec<SegmentPair>;

struct PermutationGenerator {
    counter: BigUint,
    size: usize,
}

impl PermutationGenerator {
    fn new(size: usize) -> Self {
        PermutationGenerator {
            counter: BigUint::from(0u8),
            size,
        }
    }
}

impl Iterator for PermutationGenerator {
    type Item = RowSegments;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter.bits() > self.size {
            return None;
        }

        self.counter += 1u8;

        Some(split_into_segments(&self.counter.to_str_radix(2)))
    }
}

fn split_into_segments(num_str: &str) -> RowSegments {
    let mut start: Option<usize> = None;

    num_str
        .chars()
        .rev()
        .positions(|n| n == '1')
        .batching(|iter| {
            if start == None {
                // Initial setup for the first iteration
                if let Some(idx) = iter.next() {
                    start = Some(idx);
                } else {
                    return None;
                }
            }

            let mut end = start;

            loop {
                if let Some(idx) = iter.next() {
                    if idx - end.unwrap() > 1 {
                        let pair = SegmentPair {
                            start: start.unwrap(),
                            end: end.unwrap(),
                        };
                        start = Some(idx);
                        break Some(pair);
                    } else {
                        end = Some(idx);
                    }
                } else {
                    let pair = SegmentPair {
                        start: start.unwrap(),
                        end: end.unwrap(),
                    };
                    start = None;
                    break Some(pair);
                }
            }
        })
        .collect()
}

fn row_matches_clue(clue: &[usize], row: &[SegmentPair]) -> bool {
    clue.len() == row.len()
        && clue
            .iter()
            .zip(row)
            .all(|(clue_seg, row_seg)| (row_seg.end + 1) - row_seg.start == *clue_seg)
}

fn enumerate_possible_from_clue(clue: &[usize], size: usize) -> Vec<RowSegments> {
    if clue.is_empty() {
        vec![vec![]]
    } else if clue[0] == size {
        vec![vec![SegmentPair {
            start: 0,
            end: size - 1,
        }]]
    } else {
        PermutationGenerator::new(size)
            .filter(|row| row_matches_clue(clue, row))
            .collect()
    }
}

fn is_in_segment(index: usize, segment: &SegmentPair) -> bool {
    index >= segment.start && index <= segment.end
}

fn valid_possibility(
    possible: &[SegmentPair],
    grid_iter: impl Iterator<Item = grid::CellState>,
) -> bool {
    use grid::CellState;

    grid_iter.enumerate().all(|(idx, cell)| match cell {
        CellState::Filled => possible.iter().any(|seg| is_in_segment(idx, seg)),
        CellState::Empty => !possible.iter().any(|seg| is_in_segment(idx, seg)),
        CellState::Unknown => true,
    })
}

fn common_cells(possibilities: &[RowSegments], size: usize) -> (Vec<usize>, Vec<usize>) {
    let mut common: Vec<usize> = (0..size).map(|_| 0).collect();

    for possibility in possibilities {
        for (idx, cell) in common.iter_mut().enumerate().take(size) {
            if possibility.iter().any(|seg| is_in_segment(idx, seg)) {
                *cell += 1;
            }
        }
    }

    (
        common
            .iter()
            .positions(|n| *n == possibilities.len())
            .collect(),
        common.iter().positions(|n| *n == 0).collect(),
    )
}

pub fn has_single_solution(row_clues: &[Vec<usize>], column_clues: &[Vec<usize>]) -> bool {
    use grid::{CellState, StateGrid};

    let height = row_clues.len();
    let width = column_clues.len();

    let mut all_row_possibilities: Vec<Vec<RowSegments>> = row_clues
        .iter()
        .map(|clue| enumerate_possible_from_clue(clue, width))
        .collect();
    let mut all_column_possibilities: Vec<Vec<RowSegments>> = column_clues
        .iter()
        .map(|clue| enumerate_possible_from_clue(clue, height))
        .collect();

    let mut grid = StateGrid::new(height, width);

    loop {
        let mut changes = 0;

        for (i, row_possibilities) in all_row_possibilities.iter_mut().enumerate() {
            let before = row_possibilities.len();

            row_possibilities
                .retain(|possible_row| valid_possibility(possible_row, grid.row_iter(i)));

            changes += before - row_possibilities.len();

            let (common_filled, common_empty) = common_cells(&row_possibilities, width);

            for j in common_filled {
                if grid.get(i, j) == CellState::Unknown {
                    changes += 1;
                    grid.set(i, j, CellState::Filled);
                }
            }

            for j in common_empty {
                if grid.get(i, j) == CellState::Unknown {
                    changes += 1;
                    grid.set(i, j, CellState::Empty);
                }
            }
        }

        for (j, column_possibilities) in all_column_possibilities.iter_mut().enumerate() {
            let before = column_possibilities.len();

            column_possibilities
                .retain(|possible_column| valid_possibility(possible_column, grid.column_iter(j)));

            changes += before - column_possibilities.len();

            let (common_filled, common_empty) = common_cells(&column_possibilities, height);

            for i in common_filled {
                if grid.get(i, j) == CellState::Unknown {
                    changes += 1;
                    grid.set(i, j, CellState::Filled);
                }
            }

            for i in common_empty {
                if grid.get(i, j) == CellState::Unknown {
                    changes += 1;
                    grid.set(i, j, CellState::Empty);
                }
            }
        }

        if grid.is_known() {
            break true;
        } else if changes == 0 {
            break false;
        }
    }
}
