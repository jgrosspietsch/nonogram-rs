extern crate nonogram;

extern crate ndarray;
extern crate serde_json;

use nonogram::Nonogram;

use ndarray::{arr1, arr2};

#[test]
fn serialize_puzzle() {
    let puzzle = Nonogram {
        row_segments: arr1(&[vec![], vec![1, 1], vec![], vec![1, 1], vec![]]),
        column_segments: arr1(&[vec![], vec![1, 1], vec![], vec![1, 1], vec![]]),
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ]),
    };

    let serialized = "{\"checksum\":3087051523477295210,\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}";

    assert_eq!(puzzle.as_json().unwrap(), serialized);
}

#[test]
fn deserialize_puzzle() {
    let puzzle = Nonogram {
        row_segments: arr1(&[vec![], vec![1, 1], vec![], vec![1, 1], vec![]]),
        column_segments: arr1(&[vec![], vec![1, 1], vec![], vec![1, 1], vec![]]),
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ]),
    };

    let serialized = String::from("{\"checksum\":3087051523477295210,\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        Nonogram::from_json(&serialized).unwrap().row_segments,
        puzzle.row_segments
    );
    assert_eq!(
        Nonogram::from_json(&serialized).unwrap().column_segments,
        puzzle.column_segments
    );
    assert_eq!(
        Nonogram::from_json(&serialized).unwrap().completed_grid,
        puzzle.completed_grid
    );
}

#[test]
fn deserialize_invalid_json() {
    let serialized = String::from("{\"checksum\"3087051523477295210,\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        Nonogram::from_json(&serialized).unwrap_err(),
        "expected `:` at line 1 column 12"
    );
}

#[test]
fn deserialize_invalid_object_field() {
    let serialized = String::from("{\"checksummmm\":3087051523477295210,\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        Nonogram::from_json(&serialized).unwrap_err(),
        "missing field `checksum` at line 1 column 214"
    );
}

#[test]
fn deserialize_invalid_grid_dimensions() {
    let serialized = String::from("{\"checksum\":3087051523477295210,\"height\":5,\"width\":7,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        Nonogram::from_json(&serialized).unwrap_err(),
        "ShapeError/OutOfBounds: out of bounds indexing"
    );
}

#[test]
fn deserialize_invalid_grid_rows() {
    let serialized = String::from("{\"checksum\":3087051523477295210,\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0,1],[0,0,0,0,0]]}");

    assert_eq!(
        Nonogram::from_json(&serialized).unwrap_err(),
        "ShapeError/IncompatibleShape: incompatible shapes"
    );
}
