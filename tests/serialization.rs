use nonogram::Nonogram;

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

    let serialized: String = r#"
        {
            "checksum": "3087051523477295210",
            "height": 5,
            "width": 5,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0]
            ]
        }"#
    .split_whitespace()
    .collect::<Vec<&str>>()
    .join("");

    assert_eq!(
        serde_json::to_string(&puzzle)
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(""),
        serialized
    );
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

    let serialized = r#"
        {
            "checksum": "3087051523477295210",
            "height": 5,
            "width": 5,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0]
            ]
        }"#;

    assert_eq!(
        serde_json::from_str::<Nonogram>(&serialized)
            .unwrap()
            .row_segments,
        puzzle.row_segments
    );
    assert_eq!(
        serde_json::from_str::<Nonogram>(&serialized)
            .unwrap()
            .column_segments,
        puzzle.column_segments
    );
    assert_eq!(
        serde_json::from_str::<Nonogram>(&serialized)
            .unwrap()
            .completed_grid,
        puzzle.completed_grid
    );
}

#[test]
#[should_panic]
fn deserialize_invalid_json() {
    let serialized = r#"
        {
            "checksum" "3087051523477295210",
            "height": 5,
            "width": 5,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0]
            ]
        }"#;

    serde_json::from_str::<Nonogram>(&serialized).unwrap();
}

#[test]
#[should_panic]
fn deserialize_invalid_object_field() {
    let serialized = r#"
        {
            "checksummmm": "3087051523477295210",
            "height": 5,
            "width": 5,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0]
            ]
        }"#;

    serde_json::from_str::<Nonogram>(&serialized).unwrap();
}

#[test]
#[should_panic]
fn deserialize_invalid_grid_dimensions() {
    let serialized = r#"
        {
            "checksum": "3087051523477295210",
            "height": 5,
            "width": 7,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0]
            ]
        }"#;

    serde_json::from_str::<Nonogram>(&serialized).unwrap();
}

#[test]
#[should_panic]
fn deserialize_invalid_grid_rows() {
    let serialized = r#"
        {
            "checksum": "3087051523477295210",
            "height": 5,
            "width": 5,
            "row_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],
            "column_segments": [
                [],
                [1,1],
                [],
                [1,1],
                []
            ],"completed_grid": [
                [0,0,0,0,0],
                [0,1,0,1,0],
                [0,0,0,0,0],
                [0,1,0,1,0,1],
                [0,0,0,0,0]
            ]
        }"#;

    serde_json::from_str::<Nonogram>(&serialized).unwrap();
}
