use std::fs::File;
use std::io::prelude::*;

use sudoku::{format_solution, sudoku_solver};

#[test]
fn tests() {
    let mut inputs = String::new();
    File::open("./data/95_hard_sudokus.txt")
        .unwrap()
        .read_to_string(&mut inputs)
        .unwrap();

    let mut solutions = String::new();
    File::open("./data/95_hard_sudokus_solutions.txt")
        .unwrap()
        .read_to_string(&mut solutions)
        .unwrap();

    for (input, solution) in inputs.lines().zip(solutions.lines()) {
        let (lits, mut solver) = sudoku_solver(input);
        solver.solve().unwrap();
        let result = format_solution(&solver.model().unwrap(), &lits);
        assert_eq!(result, solution);
    }
}
