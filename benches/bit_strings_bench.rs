/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::bit_strings::solve;

pub fn bit_strings_bench(c: &mut Criterion) {
    c.bench_function("bit_strings(23)", |b| b.iter(|| solve(black_box(23))));
}

criterion_group!(benches, bit_strings_bench);
criterion_main!(benches);
