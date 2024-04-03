/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::permutations::solve;

pub fn permutations_bench(c: &mut Criterion) {
    c.bench_function("permutations(23)", |b| b.iter(|| solve(black_box(23))));
}

criterion_group!(benches, permutations_bench);
criterion_main!(benches);
