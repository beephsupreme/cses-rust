/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::apple_division::solve;
use cses::utils::io::{get_int, get_vector};

fn main() {
    let n: usize = get_int().unwrap();
    let v: Vec<usize> = get_vector().unwrap();
    println!("{}", solve(n, v));
}
