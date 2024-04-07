/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::apple_division::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::{file_to_int, file_to_string};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = get_test_filenames("apple_division");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let q = file_to_string(&questions[i]);
        let a = file_to_int(&answers[i]);
        let mut tokens = q.split_ascii_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let v: Vec<usize> = tokens.map(|s| s.parse().unwrap()).collect();
        assert_eq!(solve(n, v), a);
    });
}
