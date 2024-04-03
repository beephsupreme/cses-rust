/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::gray_code::solve;

pub fn gray_code_bench(c: &mut Criterion) {
    c.bench_function("gray_code(8)", |b| b.iter(|| solve(black_box(8))));
}

criterion_group!(benches, gray_code_bench);
criterion_main!(benches);
