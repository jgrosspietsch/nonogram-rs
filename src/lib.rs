mod solution;

extern crate serde;
extern crate serde_json;

use crc::crc64::checksum_ecma;
use ndarray::{iter::Lanes, ArrayView1, Ix1};
use ndarray_rand::RandomExt;
use rand::distributions::Uniform;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error as JsonError;
use std::hash::{Hash, Hasher};

pub use ndarray::{arr1, arr2, Array1, Array2};

fn build_clue(row: ArrayView1<u8>) -> Vec<usize> {
    let mut clue: Vec<usize> = Vec::new();

    row.into_iter()
        .cloned()
        .collect::<Vec<u8>>()
        .split(|cell| *cell == 0u8)
        .for_each(|segment| {
            if !segment.is_empty() {
                clue.push(segment.len());
            }
        });

    clue
}

fn build_clues(grid: Lanes<u8, Ix1>) -> Vec<Vec<usize>> {
    grid.into_iter().map(build_clue).collect()
}

#[derive(Debug, Clone)]
pub struct Nonogram {
    pub row_segments: Vec<Vec<usize>>,
    pub column_segments: Vec<Vec<usize>>,
    pub completed_grid: Array2<u8>,
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
            completed_grid,
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
        solution::has_single_solution(&self.row_segments, &self.column_segments)
    }

    /// Generates a checksum for quickly determining equivalence between puzzles of
    /// like dimensions.
    ///
    /// This is *not* meant to check equivalence between all puzzles, only those with the
    /// same dimensions. This function is meant for cross-platform checks for equivalence in those
    /// situations.
    pub fn generate_checksum(&self) -> u64 {
        let mut aggregate: Vec<u8> = Vec::new();

        self.completed_grid
            .iter()
            .for_each(|cell| aggregate.push(*cell));

        checksum_ecma(aggregate.as_slice())
    }

    /// Serializes the nonogram as json so that we don't need to use serde every time we need to use it
    pub fn as_json(&self) -> Result<String, JsonError> {
        serde_json::to_string(&SerializedNonogram::from_nonogram(self))
    }

    pub fn from_json(serialized: &str) -> Result<Nonogram, String> {
        match serde_json::from_str::<SerializedNonogram>(serialized) {
            Ok(deserialized) => match deserialized.to_nonogram() {
                Ok(nonogram) => Ok(nonogram),
                Err(e) => Err(e.to_string()),
            },
            Err(e) => Err(e.to_string()),
        }
    }
}

impl Hash for Nonogram {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.height().hash(state);
        self.width().hash(state);
        self.generate_checksum().hash(state);
    }
}

impl PartialEq for Nonogram {
    fn eq(&self, other: &Self) -> bool {
        self.height() == other.height()
            && self.width() == other.width()
            && self.generate_checksum() == other.generate_checksum()
    }
}

impl Eq for Nonogram {}

impl Serialize for Nonogram {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerializedNonogram::from_nonogram(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Nonogram {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match SerializedNonogram::deserialize(deserializer) {
            Ok(deserialized) => Ok(deserialized.to_nonogram().unwrap()),
            Err(e) => Err(e),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializedNonogram {
    checksum: String,
    height: usize,
    width: usize,
    row_segments: Vec<Vec<usize>>,
    column_segments: Vec<Vec<usize>>,
    completed_grid: Vec<Vec<u8>>,
}

impl SerializedNonogram {
    fn from_nonogram(original: &Nonogram) -> SerializedNonogram {
        SerializedNonogram {
            checksum: original.generate_checksum().to_string(),
            height: original.height(),
            width: original.width(),
            row_segments: original.row_segments.iter().cloned().collect(),
            column_segments: original.column_segments.iter().cloned().collect(),
            completed_grid: original
                .completed_grid
                .genrows()
                .into_iter()
                .map(|row| row.iter().cloned().collect())
                .collect(),
        }
    }

    fn to_nonogram(&self) -> Result<Nonogram, String> {
        let grid = Array2::from_shape_vec(
            (self.height, self.width),
            self.completed_grid.iter().flatten().cloned().collect(),
        );

        match grid {
            Ok(grid_array) => Ok(Nonogram {
                row_segments: self.row_segments.clone(),
                column_segments: self.column_segments.clone(),
                completed_grid: grid_array,
            }),
            Err(e) => Err(e.to_string()),
        }
    }
}
