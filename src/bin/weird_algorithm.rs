/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use anyhow::Error;
use cses::solutions::weird_algorithm::solve;
use cses::utils::io::{get_bytes, get_int, vector_to_string};

/// Driver for the "Weird Algorithm" problem https://cses.fi/problemset/task/1068
fn main() -> Result<(), Error> {
    let reader = std::io::BufReader::new(std::io::stdin());
    let input: Vec<u8> = get_bytes(reader)?;
    let n: u64 = get_int(input)?;
    let r: Vec<u64> = solve(n)?;
    println!("{}", vector_to_string(r, Some(" ")));
    Ok(())
}
