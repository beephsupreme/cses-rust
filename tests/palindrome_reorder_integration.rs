/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::palindrome_reorder::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::file_to_string;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = get_test_filenames("palindrome_reorder");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let q = file_to_string(&questions[i]);
        let a = file_to_string(&answers[i]);
        let r: Option<String> = solve(q);
        match r {
            Some(x) => assert_eq!(x, a),
            None => assert_eq!(a, "NO SOLUTION"),
        }
    });
}
