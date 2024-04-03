/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fmt::Write;

use cses::solutions::two_sets::solve;
use cses::utils::io::get_int;

fn main() {
    let n: u64 = get_int();
    let mut output = String::with_capacity(6888916);
    match solve(n) {
        Some((a, b)) => {
            writeln!(output, "YES").unwrap();
            writeln!(output, "{}", a.len()).unwrap();
            a.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output, "\n{}", b.len()).unwrap();
            b.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output).unwrap();
        }
        None => writeln!(output, "NO").unwrap(),
    }
    println!("{}", output);
}
