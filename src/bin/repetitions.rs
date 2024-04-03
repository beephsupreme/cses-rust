/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::repetitions::solve;
use cses::utils::io::get_string;

fn main() {
    let s: String = get_string();
    let n: u64 = solve(s);
    println!("{}", n);
}
