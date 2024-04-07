/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use anyhow::Result;
use log::info;

use cses::solutions::weird_algorithm::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::{get_token, get_vector, load_all_tokens};

#[test]
fn weird_algorithm_integration() -> Result<()> {
    env_logger::init();
    let (questions, answers) = get_test_filenames("weird_algorithm");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = std::io::BufReader::new(std::fs::File::open(&questions[i]).unwrap());
        let mut a_reader = std::io::BufReader::new(std::fs::File::open(&answers[i]).unwrap());
        let mut q_buffer: String = String::new();
        let mut a_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer).unwrap();
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer).unwrap();
        let n: u64 = get_token(&mut q_tokens).unwrap();
        let a: Vec<u64> = get_vector(&mut a_tokens).unwrap();
        let r: Vec<u64> = solve(n).unwrap();
        assert_eq!(a, r);
    });
    Ok(())
}
