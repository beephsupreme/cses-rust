/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::trailing_zeros::solve;
use cses::utils::io::get_int;

fn main() {
    let n: u64 = get_int();
    let r = solve(n);
    println!("{}", r);
}
