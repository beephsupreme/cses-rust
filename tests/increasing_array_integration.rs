/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::increasing_array::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string};

#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("increasing_array");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let question: String = file_to_string(&questions[i]);
        let mut q_tokens = question.split_ascii_whitespace();
        let _ = q_tokens.next().unwrap();
        let vector: Vec<u64> = q_tokens.map(|s| s.parse().unwrap()).collect();
        let a: u64 = file_to_int(&answers[i]);
        assert_eq!(solve(vector), a);
    });
}
