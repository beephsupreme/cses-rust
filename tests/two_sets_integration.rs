/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::two_sets::solve;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::file_to_int;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, _) = get_test_filenames("two_sets");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let n: u64 = file_to_int(&questions[i]);
        if let Some((a, b)) = solve(n) {
            assert_eq!(n, (a.len() + b.len()) as u64);
            assert_eq!(
                n * (n + 1) / 2,
                a.iter().sum::<u64>() + b.iter().sum::<u64>()
            );
            assert_eq!(a.iter().sum::<u64>(), b.iter().sum::<u64>());
        } else {
            assert!(n % 4 == 1 || n % 4 == 2);
        }
    });
}
