/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::gray_code::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("gray_code");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let n: u64 = file_to_int(&questions[i]);
        let answer: String = file_to_string(&answers[i]);
        let mut a: Vec<String> = answer.lines().map(|x| x.to_string()).collect();
        let mut r: Vec<String> = solve(n);
        r.sort();
        a.sort();
        assert_eq!(r, a);
    });
}
