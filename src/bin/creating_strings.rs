/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use cses::solutions::creating_strings::solve;
use cses::utils::io::{get_string, vector_to_string};

fn main() {
    let s: String = get_string().unwrap();
    let r: Vec<String> = solve(s);
    println!("{}", r.len());
    print!("{}", vector_to_string(r, Some("\n")));
}
