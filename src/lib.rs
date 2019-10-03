use std::char;

use varisat::{CnfFormula, ExtendFormula, Lit, Solver};

const ROWS: usize = 9;
const COLUMNS: usize = 9;
const NUMBERS: usize = 9;

#[derive(Debug)]
struct Entry {
    value: usize,
    row: usize,
    column: usize,
}

impl From<Entry> for usize {
    fn from(source: Entry) -> Self {
        9 * 9 * source.value + 9 * source.row + source.column
    }
}

fn no_row_contains_duplicates(solver: &mut Solver, lits: &Vec<Lit>) {
    for value in 0..NUMBERS {
        for row in 0..ROWS {
            let mut formula_lits = Vec::with_capacity(9);
            for column in 0..COLUMNS {
                let entry = Entry { value, row, column };
                formula_lits.push(lits[usize::from(entry)]);
            }
            solver.add_formula(&exactly_one_true(&formula_lits));
        }
    }
}

fn no_column_contains_duplicates(solver: &mut Solver, lits: &Vec<Lit>) {
    for value in 0..NUMBERS {
        for column in 0..COLUMNS {
            let mut formula_lits = Vec::with_capacity(9);
            for row in 0..ROWS {
                let entry = Entry { value, row, column };
                formula_lits.push(lits[usize::from(entry)]);
            }
            solver.add_formula(&exactly_one_true(&formula_lits));
        }
    }
}

fn no_box_contains_duplicates(solver: &mut Solver, lits: &Vec<Lit>) {
    for value in 0..NUMBERS {
        for row in (0..=6).step_by(3) {
            for column in (0..=6).step_by(3) {
                let mut formula_lits = Vec::with_capacity(9);
                for box_row in 0..=2 {
                    for box_column in 0..=2 {
                        let entry = Entry {
                            value,
                            row: row + box_row,
                            column: column + box_column,
                        };
                        formula_lits.push(lits[usize::from(entry)]);
                    }
                }
                solver.add_formula(&exactly_one_true(&formula_lits));
            }
        }
    }
}

fn every_field_contains_one_number(solver: &mut Solver, lits: &Vec<Lit>) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mut formula_lits = Vec::with_capacity(9);
            for value in 0..NUMBERS {
                let entry = Entry { value, row, column };
                formula_lits.push(lits[usize::from(entry)]);
            }
            solver.add_formula(&exactly_one_true(&formula_lits));
        }
    }
}

fn exactly_one_true(lits: &Vec<Lit>) -> CnfFormula {
    let mut formula = CnfFormula::new();
    formula.add_clause(&lits);
    for i in 0..lits.len() {
        for j in i + 1..lits.len() {
            formula.add_clause(&[!lits[i], !lits[j]]);
        }
    }
    formula
}

fn add_board_entries(solver: &mut Solver, lits: &Vec<Lit>, board: &str) {
    let mut formula = CnfFormula::new();
    for (i, elem) in board.chars().enumerate() {
        if elem.is_digit(10) {
            let entry = Entry {
                value: elem.to_digit(10).unwrap() as usize - 1,
                row: i / 9,
                column: i % 9,
            };
            formula.add_clause(&[lits[usize::from(entry)]]);
        }
    }
    solver.add_formula(&formula);
}

pub fn sudoku_solver(board: &str) -> (Vec<Lit>, Solver) {
    let mut solver = Solver::new();
    let lits: Vec<Lit> = solver.new_lit_iter(ROWS * COLUMNS * NUMBERS).collect();

    no_row_contains_duplicates(&mut solver, &lits);
    no_column_contains_duplicates(&mut solver, &lits);
    no_box_contains_duplicates(&mut solver, &lits);
    every_field_contains_one_number(&mut solver, &lits);
    add_board_entries(&mut solver, &lits, board);

    (lits, solver)
}

pub fn format_solution(model: &Vec<Lit>, lits: &Vec<Lit>) -> String {
    let mut s = String::with_capacity(ROWS * COLUMNS);
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            for value in 0..NUMBERS {
                let entry = Entry { value, row, column };
                if model.contains(&lits[usize::from(entry)]) {
                    s.push(char::from_digit((value + 1) as u32, 10).unwrap());
                }
            }
        }
    }
    s
}

pub fn print_board(board: &str) {
    for row in 0..ROWS {
        println!("{}", &board[COLUMNS * row..COLUMNS * (row + 1)])
    }
}
