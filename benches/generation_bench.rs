#![feature(test)]
extern crate test;

use nonogram::Nonogram;
use test::{Bencher};

#[bench]
fn generate_and_solve_1000_5_by_5(b: &mut Bencher) {
    b.iter(|| Nonogram::generate(5, 5).solvable());
}

#[bench]
fn generate_and_solve_1000_10_by_10(b: &mut Bencher) {
    b.iter(|| Nonogram::generate(10, 10).solvable());
}

#[bench]
fn generate_and_solve_1000_15_by_15(b: &mut Bencher) {
    b.iter(|| Nonogram::generate(15, 15).solvable());
}
