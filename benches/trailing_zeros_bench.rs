/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::trailing_zeros::solve;

pub fn trailing_zeros_bench(c: &mut Criterion) {
    c.bench_function("trailing_zeros(23)", |b| b.iter(|| solve(black_box(23))));
}

criterion_group!(benches, trailing_zeros_bench);
criterion_main!(benches);
