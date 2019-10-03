#[macro_use]
extern crate criterion;

use criterion::BenchmarkId;
use criterion::Criterion;

use std::fs::File;
use std::io::prelude::*;

use sudoku::{format_solution, sudoku_solver};

fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("95 hard sudokus");

    let mut inputs = String::new();
    File::open("./data/95_hard_sudokus.txt")
        .unwrap()
        .read_to_string(&mut inputs)
        .unwrap();

    for (index, input) in inputs.lines().enumerate() {
        group.bench_with_input(
            BenchmarkId::from_parameter(index + 1),
            &input,
            move |b, s| {
                b.iter(|| {
                    let (lits, mut solver) = sudoku_solver(&s);
                    solver.solve().unwrap();
                    format_solution(&solver.model().unwrap(), &lits)
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
