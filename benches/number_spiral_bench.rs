/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{criterion_group, criterion_main, Criterion};

use cses::solutions::number_spiral::solve;

pub fn number_spiral_bench(c: &mut Criterion) {
    c.bench_function("number_spiral(vec![(2, 3), (1, 1), (4, 2)])", |b| {
        b.iter(|| solve(3, vec![(2, 3), (1, 1), (4, 2)]))
    });
}

criterion_group!(benches, number_spiral_bench);
criterion_main!(benches);
