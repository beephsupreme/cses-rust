/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::repetitions::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string};

#[cfg(test)]
#[test]

fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("repetitions");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let question: String = file_to_string(&questions[i]);
        let answer: u64 = file_to_int(&answers[i]);
        assert_eq!(solve(question), answer);
    });
}
