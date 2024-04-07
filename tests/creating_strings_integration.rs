/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::creating_strings::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::file_to_string;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = get_test_filenames("creating_strings");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let answer: String = file_to_string(&answers[i]);
        let mut answer = answer.split_ascii_whitespace();
        let _: u64 = answer.next().unwrap().parse().unwrap();
        let a = answer.map(|s| s.to_string()).collect::<Vec<String>>();
        let q = file_to_string(&questions[i]);
        let r = solve(q);
        assert_eq!(r, a);
    });
}
