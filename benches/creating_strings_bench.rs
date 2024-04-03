/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use cses::solutions::creating_strings::solve;

pub fn creating_strings_bench(c: &mut Criterion) {
    c.bench_function("creating_strings(ABCDEFG)", |b| {
        b.iter(|| solve(black_box("ABCDEFG".to_string())))
    });
}

criterion_group!(benches, creating_strings_bench);
criterion_main!(benches);
