/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::gray_code::solve;
use cses::utils::io::{get_int, vector_to_string};

fn main() {
    let n: u64 = get_int();
    let r: Vec<String> = solve(n);
    print!("{}", vector_to_string(r, Some("\n")));
}
