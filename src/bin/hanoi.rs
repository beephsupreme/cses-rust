/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::tower_of_hanoi::solve;
use cses::utils::io::get_int;

fn main() {
    let n: u8 = get_int();
    let v: Vec<(u8, u8)> = solve(n);
    println!("{}", v.len());
    v.into_iter().for_each(|(a, b)| println!("{} {}", a, b));
}
