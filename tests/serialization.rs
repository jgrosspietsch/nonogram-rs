use nonogram::Nonogram;
use serde_json::{from_str, to_string};

use ndarray::arr2;

#[test]
fn serialize_puzzle() {
    let puzzle = Nonogram {
        row_segments: vec![vec![], vec![1, 1], vec![], vec![1, 1], vec![]],
        column_segments: vec![vec![], vec![1, 1], vec![], vec![1, 1], vec![]],
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ]),
    };

    let serialized = "{\"checksum\":\"3087051523477295210\",\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}";

    assert_eq!(to_string(&puzzle).unwrap(), serialized);
}

#[test]
fn deserialize_puzzle() {
    let puzzle = Nonogram {
        row_segments: vec![vec![], vec![1, 1], vec![], vec![1, 1], vec![]],
        column_segments: vec![vec![], vec![1, 1], vec![], vec![1, 1], vec![]],
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
        ]),
    };

    let serialized = String::from("{\"checksum\":\"3087051523477295210\",\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap().row_segments,
        puzzle.row_segments
    );
    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap().column_segments,
        puzzle.column_segments
    );
    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap().completed_grid,
        puzzle.completed_grid
    );
}

#[test]
fn deserialize_invalid_json() {
    let serialized = String::from("{\"checksum\"\"3087051523477295210\",\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap_err().to_string(),
        "expected `:` at line 1 column 12"
    );
}

#[test]
fn deserialize_invalid_object_field() {
    let serialized = String::from("{\"checksummmm\":\"3087051523477295210\",\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap_err().to_string(),
        "missing field `checksum` at line 1 column 216"
    );
}

#[test]
fn deserialize_invalid_grid_dimensions() {
    let serialized = String::from("{\"checksum\":\"3087051523477295210\",\"height\":5,\"width\":7,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0]]}");

    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap_err().to_string(),
        "ShapeError/OutOfBounds: out of bounds indexing"
    );
}

#[test]
fn deserialize_invalid_grid_rows() {
    let serialized = String::from("{\"checksum\":\"3087051523477295210\",\"height\":5,\"width\":5,\"row_segments\":[[],[1,1],[],[1,1],[]],\"column_segments\":[[],[1,1],[],[1,1],[]],\"completed_grid\":[[0,0,0,0,0],[0,1,0,1,0],[0,0,0,0,0],[0,1,0,1,0,1],[0,0,0,0,0]]}");

    assert_eq!(
        from_str::<Nonogram>(&serialized).unwrap_err().to_string(),
        "ShapeError/IncompatibleShape: incompatible shapes"
    );
}
