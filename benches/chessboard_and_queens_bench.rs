/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::chessboard_and_queens::solve;

pub fn chessboard_and_queens_bench(c: &mut Criterion) {
    let v: Vec<Vec<bool>> = vec![
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
        vec![true, true, true, true, true, true, true, true],
    ];
    c.bench_function("chessboard_and_queens(0, &mut [0usize; 8],&b)", |b| {
        b.iter(|| solve(black_box(0), &mut [0usize; 8], &v));
    });
}

criterion_group!(benches, chessboard_and_queens_bench);
criterion_main!(benches);
