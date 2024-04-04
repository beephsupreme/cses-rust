/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::palindrome_reorder::solve;
use cses::utils::io::get_string;

fn main() {
    let s = get_string().unwrap();
    let r = solve(s);
    match r {
        Some(x) => println!("{}", x),
        None => println!("NO SOLUTION"),
    }
}
