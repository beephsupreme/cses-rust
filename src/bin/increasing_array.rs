/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::increasing_array::solve;
use cses::utils::io::{get_int, get_vector};

fn main() {
    let _: u64 = get_int().unwrap();
    let v: Vec<u64> = get_vector();
    println!("{}", solve(v))
}
