/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */
use std::fmt::Write;
use std::io::BufReader;

use anyhow::Result;

use cses::solutions::tower_of_hanoi::tower_of_hanoi_solution;
use cses::utils::io::{get_token, load_tokens};

fn main() -> Result<()> {
    let reader = BufReader::new(std::io::stdin());
    let mut buffer: String = String::new();
    let mut tokens = load_tokens(reader, &mut buffer)?;
    let n: u8 = get_token(&mut tokens)?;
    let r: Vec<(u8, u8)> = tower_of_hanoi_solution(n)?;
    let mut output = String::new();
    writeln!(output, "{}", r.len())?;
    for (a, b) in r {
        writeln!(output, "{} {}", a, b)?;
    }
    println!("{}", output);
    Ok(())
}
