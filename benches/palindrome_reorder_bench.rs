/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{criterion_group, criterion_main, Criterion};

use cses::solutions::palindrome_reorder::solve;

pub fn palindrome_reorder_bench(c: &mut Criterion) {
    c.bench_function("palindrome_reorder(AAABBCAAAAA)", |b| {
        b.iter(|| solve("AAABBCAAAAA".to_string()))
    });
}

criterion_group!(benches, palindrome_reorder_bench);
criterion_main!(benches);
