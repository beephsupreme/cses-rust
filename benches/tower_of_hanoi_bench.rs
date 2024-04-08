/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use cses::solutions::tower_of_hanoi::tower_of_hanoi_solution;

pub fn tower_of_hanoi_bench(c: &mut Criterion) {
    c.bench_function("tower_of_hanoi(8)", |b| {
        b.iter(|| tower_of_hanoi_solution(black_box(8)))
    });
}

criterion_group!(benches, tower_of_hanoi_bench);
criterion_main!(benches);
