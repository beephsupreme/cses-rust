/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::coin_piles::solve;

pub fn coin_piles_bench(c: &mut Criterion) {
    c.bench_function("coin_piles(3, vec![(2, 1), (2, 2), (3, 3)])", |b| {
        b.iter(|| solve(black_box(3), vec![(2, 1), (2, 2), (3, 3)]))
    });
}

criterion_group!(benches, coin_piles_bench);
criterion_main!(benches);
