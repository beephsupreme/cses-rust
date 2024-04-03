/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::repetitions::solve;

pub fn repetitions_bench(c: &mut Criterion) {
    c.bench_function("repetitions(AAABBGGGGTT)", |b| {
        b.iter(|| solve(black_box("AAABBGGGGTT".to_string())))
    });
}

criterion_group!(benches, repetitions_bench);
criterion_main!(benches);
