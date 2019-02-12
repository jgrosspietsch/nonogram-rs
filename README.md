# Nonogram

[![Crate](https://img.shields.io/crates/v/nonogram.svg)](https://crates.io/crates/nonogram)
[![Docs](https://docs.rs/nonogram/badge.svg)](https://docs.rs/nonogram)

This is a Rust crate intended to generate and solve nonogram puzzles.

The core data structure used for the generated nonograms requires the use of [ndarray](https://crates.io/crates/ndarray) since the data used is primarily two-dimensional.