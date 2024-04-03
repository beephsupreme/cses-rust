/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::chessboard_and_queens::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_vector_vector_bool};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("chessboard_and_queens");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let b: Vec<Vec<bool>> = file_to_vector_vector_bool(&questions[i]);
        let a: usize = file_to_int(&answers[i]);
        let r: usize = solve(0, &mut [0usize; 8], &b);
        assert_eq!(r, a);
    });
}
