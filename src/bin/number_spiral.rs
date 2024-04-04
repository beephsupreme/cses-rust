/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::number_spiral::solve;
use cses::utils::io::{get_int, get_tuple_vector, vector_to_string};

fn main() {
    let n: u64 = get_int().unwrap();
    let v: Vec<(u64, u64)> = get_tuple_vector(n);
    let u: Vec<u64> = solve(n, v);
    print!("{}", vector_to_string(u, Some("\n")));
}
