/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::two_knights::solve;
use cses::utils::io::{get_int, vector_to_string};

fn main() {
    let n: i64 = get_int();
    let v: Vec<i64> = solve(n);
    print!("{}", vector_to_string(v, Some("\n")));
}
