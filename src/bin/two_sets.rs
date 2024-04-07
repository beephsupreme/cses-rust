/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fmt::Write;

use anyhow::{Error, Result};

use cses::solutions::two_sets::solve;

fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::io::stdin());
    let mut buffer = String::new();
    let mut tokens = cses::utils::io::load_all_tokens(reader, &mut buffer).unwrap();
    let n: u64 = cses::utils::io::get_token(&mut tokens).unwrap();
    let mut output = String::new();
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
    Ok(())
}
