/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::two_knights::solve;

pub fn two_knights_bench(c: &mut Criterion) {
    c.bench_function("two_knights(23)", |b| b.iter(|| solve(black_box(23))));
}

criterion_group!(benches, two_knights_bench);
criterion_main!(benches);
