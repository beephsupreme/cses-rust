/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::solutions::two_knights::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::{get_token, get_vector, load_all_tokens};

#[cfg(test)]
#[test]
fn two_knights_integration() -> Result<()> {
    env_logger::init();
    let (questions, answers) = get_test_filenames("two_knights");
    for i in 0..questions.len() {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = BufReader::new(File::open(&questions[i])?);
        let mut a_reader = BufReader::new(File::open(&answers[i])?);
        let mut q_buffer: String = String::new();
        let mut a_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer)?;
        let n: i64 = get_token(&mut q_tokens)?;
        let a: Vec<i64> = get_vector(&mut a_tokens)?;
        let r: Vec<i64> = solve(n)?;
        assert_eq!(a, r);
    }
    Ok(())
}
