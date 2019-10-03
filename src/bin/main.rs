use std::env;

use sudoku::{format_solution, print_board, sudoku_solver};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("sudoku BOARD");
        return;
    }
    let board = &args[1];
    if board.len() != 81 {
        println!("BOARD must have a length of 81");
    }

    let (lits, mut solver) = sudoku_solver(&board[..]);

    match solver.solve() {
        Ok(result) => match result {
            true => {
                println!("SAT:");
                let solution = format_solution(&solver.model().unwrap(), &lits);
                print_board(&solution[..]);
            }
            false => {
                println!("UNSAT");
            }
        },
        Err(err) => {
            println!("ERROR: {}", err);
        }
    }
}
