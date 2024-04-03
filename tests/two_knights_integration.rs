/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::two_knights::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string, string_to_vector};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("two_knights");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let n: i64 = file_to_int(&questions[i]);
        let a: String = file_to_string(&answers[i]);
        let v: Vec<i64> = string_to_vector(a);
        assert_eq!(v, solve(n));
    });
}
