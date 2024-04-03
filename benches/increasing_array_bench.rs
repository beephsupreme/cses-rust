/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::increasing_array::solve;

pub fn increasing_array_bench(c: &mut Criterion) {
    c.bench_function("increasing_array(vec![3, 2, 5, 1, 7])", |b| {
        b.iter(|| solve(black_box(vec![3, 2, 5, 1, 7])))
    });
}

criterion_group!(benches, increasing_array_bench);
criterion_main!(benches);
