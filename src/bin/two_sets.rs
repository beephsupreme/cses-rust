/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fmt::Write;
use std::io::BufReader;

use anyhow::{Error, Result};

use cses::solutions::two_sets::solve;
use cses::utils::io::{get_token, load_tokens};

fn main() -> Result<(), Error> {
    let reader = BufReader::new(std::io::stdin());
    let mut buffer = String::new();
    let mut tokens = load_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let mut output = String::new();
    match solve(n) {
        Some((a, b)) => {
            writeln!(output, "YES")?;
            writeln!(output, "{}", a.len())?;
            a.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output, "\n{}", b.len())?;
            b.iter().for_each(|e| write!(output, "{} ", e).unwrap());
            writeln!(output).unwrap();
        }
        None => writeln!(output, "NO")?,
    }
    println!("{}", output);
    Ok(())
}
