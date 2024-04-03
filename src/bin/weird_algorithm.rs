/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::weird_algorithm::solve;
use cses::utils::io::{get_int, vector_to_string};

/// Driver for the solution to the "Weird Algorithm" problem https://cses.fi/problemset/task/1068
fn main() {
    let n: u64 = get_int();
    let v: Vec<u64> = solve(n);
    println!("{} ", vector_to_string(v, Some(" ")));
}
