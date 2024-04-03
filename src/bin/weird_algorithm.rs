/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::weird_algorithm::solve;
use cses::utils::io::{get_int, vector_to_string};

/// Driver for the "Weird Algorithm" problem https://cses.fi/problemset/task/1068
fn main() {
    let n: u64 = get_int();
    let v= solve(n);
    match v {
        Ok(v) => println!("{} ", vector_to_string(v, Some(" "))),
        Err(e) => eprintln!("{}", e),
    }
}
