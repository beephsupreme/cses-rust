/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::apple_division::solve;

pub fn apple_division_bench(c: &mut Criterion) {
    c.bench_function("apple_division(5, vec![1, 2, 3, 4, 5])", |b| {
        b.iter(|| solve(black_box(5), vec![1, 2, 3, 4, 5]))
    });
}

criterion_group!(benches, apple_division_bench);
criterion_main!(benches);
