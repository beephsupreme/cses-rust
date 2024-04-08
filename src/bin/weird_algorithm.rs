/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::io::BufReader;

use anyhow::Error;

use cses::solutions::weird_algorithm::solve;
use cses::utils::io::{get_token, load_tokens, vector_to_string};

/// Driver for the "Weird Algorithm" problem https://cses.fi/problemset/task/1068
fn main() -> Result<(), Error> {
    let reader = BufReader::new(std::io::stdin());
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let r: Vec<u64> = solve(n)?;
    let s: String = vector_to_string(r, Some(" "));
    println!("{}", s);
    Ok(())
}
