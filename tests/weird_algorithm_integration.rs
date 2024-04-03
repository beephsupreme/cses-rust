/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::weird_algorithm;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string, vector_to_string};

#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("weird_algorithm");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let a = file_to_string(&answers[i]);
        let n: u64 = file_to_int(&questions[i]);
        let r: String = vector_to_string(weird_algorithm::solve(n).unwrap(), Some(" "));
        assert_eq!(a, r);
    });
}
