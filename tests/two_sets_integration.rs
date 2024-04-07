/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::solutions::two_sets::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::{get_token, load_all_tokens};

#[cfg(test)]
#[test]
fn two_sets_integration() -> Result<()> {
    env_logger::init();
    let (questions, _) = get_test_filenames("two_sets");
    for (i, question) in questions.iter().enumerate() {
        info!("{}: {}", i + 1, question);
        let mut q_reader = BufReader::new(std::fs::File::open(question)?);
        let mut q_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let n: u64 = get_token(&mut q_tokens)?;
        if let Some((a, b)) = solve(n) {
            assert_eq!(n, (a.len() + b.len()) as u64);
            assert_eq!(
                n * (n + 1) / 2,
                a.iter().sum::<u64>() + b.iter().sum::<u64>()
            );
            assert_eq!(a.iter().sum::<u64>(), b.iter().sum::<u64>());
        } else {
            assert!(n % 4 == 1 || n % 4 == 2);
        }
    }
    Ok(())
}
