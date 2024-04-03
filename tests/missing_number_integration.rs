/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::missing_number::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("missing_number");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let a: u64 = file_to_int(&answers[i]);
        let q = file_to_string(&questions[i]);
        let mut tokens = q.split_ascii_whitespace();
        let n: u64 = tokens.next().unwrap().parse().unwrap();
        let v: Vec<u64> = tokens.map(|s| s.parse().unwrap()).collect();
        assert_eq!(solve(n, v), a);
    });
}
