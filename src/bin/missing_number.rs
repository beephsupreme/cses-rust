/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use anyhow::Error;

use cses::solutions::missing_number::solve;
use cses::utils::io::{get_token, get_vector, load_all_tokens};

fn main() -> Result<(), Error> {
    // let f = std::fs::File::open("data/missing_number/test_input_003.txt")?;
    // let reader = std::io::BufReader::new(f);
    let reader = std::io::BufReader::new(std::io::stdin());
    // let reader = std::io::Cursor::new("42");
    let mut buffer: String = String::new();
    let mut tokens = load_all_tokens(reader, &mut buffer)?;
    let n: u64 = get_token(&mut tokens)?;
    let v: Vec<u64> = get_vector(&mut tokens)?;
    let r: u64 = solve(n, v)?;
    println!("{}", r);
    Ok(())
}
