/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::two_sets::solve;

pub fn two_sets_bench(c: &mut Criterion) {
    c.bench_function("two_sets(7)", |b| b.iter(|| solve(black_box(7))));
}

criterion_group!(benches, two_sets_bench);
criterion_main!(benches);
