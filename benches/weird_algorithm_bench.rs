/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::weird_algorithm::solve;

pub fn weird_algorithm_bench(c: &mut Criterion) {
    c.bench_function("weird_algorithm::solve(7)", |b| {
        b.iter(|| solve(black_box(7)).unwrap())
    });
}

criterion_group!(benches, weird_algorithm_bench);
criterion_main!(benches);
