mod state;

extern crate crc;
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;

use std::hash::{Hash, Hasher};
use rand::distributions::Uniform;
use ndarray::{ArrayView1, Ix1, iter::Lanes};
use ndarray_rand::RandomExt;
use crc::crc64::checksum_ecma;

pub use ndarray::{Array1, Array2, arr1, arr2};

use state::{StateRow, StateGrid, enumerate_row_states, common_row_indexes, filter_invalid_row_states};

fn build_clue(row: ArrayView1<u8>) -> Vec<usize> {
    let mut clue: Vec<usize> = Vec::new();

    if let Some(row_slice) = row.as_slice() {
        row_slice.split(|cell| *cell == 0u8).for_each(|segment| {
            if !segment.is_empty() {
                clue.push(segment.len());
            }
        });
    }

    clue
}

fn build_clues(grid: Lanes<u8, Ix1>) -> Array1<Vec<usize>> {
    grid.into_iter()
        .map(build_clue)
        .collect()
}

#[derive(Debug)]
pub struct Nonogram {
    pub row_segments: Array1<Vec<usize>>,
    pub column_segments: Array1<Vec<usize>>,
    pub completed_grid: Array2<u8>
}

/// `row_segments` and `column_segments` are in the form of a 
impl Nonogram {
    /// Generates a random nonogram with the given dimensions.
    /// 
    /// The generated puzzle is not checked for solvability.
    pub fn generate(width: usize, height: usize) -> Nonogram {
        let completed_grid = Array2::random((height, width), Uniform::new_inclusive(0, 1));

        Nonogram {
            row_segments: build_clues(completed_grid.genrows()),
            column_segments: build_clues(completed_grid.gencolumns()),
            completed_grid
        }
    }

    /// Provides the height of the puzzle.
    pub fn height(&self) -> usize {
        self.completed_grid.dim().0
    }

    /// Provides the width of the puzzle.
    pub fn width(&self) -> usize {
        self.completed_grid.dim().1
    }

    /// Determines whether or not the puzzle is solvable.
    /// 
    /// This method attempts to programmatically solve the puzzle. If it reaches a dead-end the
    /// method returns false. Otherwise it reaches the conclusion of the puzzle and returns true.
    pub fn solvable(&self) -> bool {
        let mut row_possibilities: Vec<Vec<StateRow>> = self.row_segments.iter()
            .map(|clue| enumerate_row_states(self.completed_grid.dim().0, &clue))
            .collect();

        let mut column_possibilities: Vec<Vec<StateRow>> = self.column_segments.iter()
            .map(|clue| enumerate_row_states(self.completed_grid.dim().1, &clue))
            .collect();

        let mut grid = StateGrid::new(self.height(), self.width());

        loop {
            let mut changes = 0;

            for (index, possibilities) in row_possibilities.iter_mut().enumerate() {
                let common = common_row_indexes(&possibilities);

                println!("common for row {:?}: {:?}", index, common);

                for cell in &common {
                    if let Some(current_cell) = grid.get(index, cell.0) {
                        if *current_cell != cell.1 {
                            grid.set(index, cell.0, cell.1);
                            changes += 1;
                        }
                    }
                }

                println!("grid row {:?}: {:?}", index, grid.get_row(index));

                let filtered_possibilities = filter_invalid_row_states(&grid.get_row(index), &possibilities);

                println!("filtered possibilities {:?}", filtered_possibilities);
                changes += possibilities.len() - filtered_possibilities.len();
                *possibilities = filtered_possibilities;
            }

            for (index, possibilities) in column_possibilities.iter_mut().enumerate() {
                let common = common_row_indexes(&possibilities);

                for cell in &common {
                    if let Some(current_cell) = grid.get(cell.0, index) {
                        if *current_cell != cell.1 {
                            grid.set(cell.0, index, cell.1);
                            changes += 1;
                        }
                    }
                }

                let filtered_possibilities = filter_invalid_row_states(&grid.get_column(index), &possibilities);

                changes += possibilities.len() - filtered_possibilities.len();
                *possibilities = filtered_possibilities;
            }

            if changes == 0 {
                break false;
            }

            if grid.is_known() {
                break true;
            }
        }
    }

    /// Generates a checksum for quickly determining equivalence between puzzles of
    /// like dimensions.
    /// 
    /// This is *not* meant to check equivalence between all puzzles, only those with the
    /// same dimensions. This function is meant for cross-platform checks for equivalence in those
    /// situations.
    pub fn generate_checksum(&self) -> u64 {
        let mut aggregate: Vec<u8> = Vec::new();

        self.completed_grid.iter().for_each(|cell| aggregate.push(*cell));

        checksum_ecma(aggregate.as_slice())
    }
}

impl Hash for Nonogram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.height().hash(state);
        self.width().hash(state);
        self.generate_checksum().hash(state);
    }
}
