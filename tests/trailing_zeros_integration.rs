/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::trailing_zeros::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::file_to_int;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = get_test_filenames("trailing_zeros");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let a: u64 = file_to_int(&answers[i]);
        let n: u64 = file_to_int(&questions[i]);
        let r: u64 = solve(n);
        assert_eq!(r, a);
    });
}
