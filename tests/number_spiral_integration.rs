/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::number_spiral::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::file_to_string;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("number_spiral");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let question = file_to_string(&questions[i]);
        let mut question = question.split_ascii_whitespace();
        let n: u64 = question.next().unwrap().parse().unwrap();
        let q: Vec<(u64, u64)> = question
            .map(|e| e.parse().unwrap())
            .collect::<Vec<u64>>()
            .chunks(2)
            .map(|e| (e[0], e[1]))
            .collect();
        let answer: String = file_to_string(&answers[i]);
        let a: Vec<u64> = answer
            .split_ascii_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let r: Vec<u64> = solve(n, q);
        assert_eq!(r, a);
    });
}
