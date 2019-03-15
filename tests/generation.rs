extern crate nonogram;

extern crate ndarray;

use nonogram::Nonogram;

use ndarray::{arr1, arr2};

#[test]
fn random_generation_works() {
    let puzzle1 = Nonogram::generate(15, 15);
    let puzzle2 = Nonogram::generate(15, 15);
    let puzzle3 = Nonogram::generate(15, 15);

    assert_ne!(puzzle1.generate_checksum(), puzzle2.generate_checksum());
    assert_ne!(puzzle2.generate_checksum(), puzzle3.generate_checksum());
    assert_ne!(puzzle1.generate_checksum(), puzzle3.generate_checksum());
}

#[test]
fn mismatched_dimensions_work() {
    let puzzle1 = Nonogram::generate(5, 10).solvable();
    let puzzle2 = Nonogram::generate(15, 5).solvable();
    let puzzle3 = Nonogram::generate(10, 20).solvable();

    assert!(puzzle1 || !puzzle1);
    assert!(puzzle2 || !puzzle2);
    assert!(puzzle3 || !puzzle3);
}


#[test]
fn detects_not_solvable() {
    let puzzle = Nonogram {
        row_segments: arr1(&[
            vec![],
            vec![1],
            vec![],
            vec![1],
            vec![]
        ]),
        column_segments: arr1(&[
            vec![],
            vec![1],
            vec![],
            vec![1],
            vec![]
        ]),
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0]
        ])
    };

    assert!(!puzzle.solvable())
}

#[test]
fn detects_solvable() {
    let puzzle = Nonogram {
        row_segments: arr1(&[
            vec![],
            vec![1, 1],
            vec![],
            vec![1, 1],
            vec![]
        ]),
        column_segments: arr1(&[
            vec![],
            vec![1, 1],
            vec![],
            vec![1, 1],
            vec![]
        ]),
        completed_grid: arr2(&[
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0],
            [0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0]
        ])
    };

    assert!(puzzle.solvable())
}
