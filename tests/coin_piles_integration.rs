/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::coin_piles::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::file_to_string;

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("coin_piles");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let answer: String = file_to_string(&answers[i]);
        let a = answer.split_ascii_whitespace();
        let a: Vec<String> = a.map(|s| s.to_string()).collect();
        let question: String = file_to_string(&questions[i]);
        let mut q = question.split_ascii_whitespace();
        let n: u64 = q.next().unwrap().parse().unwrap();
        let v: Vec<(u64, u64)> = (0..n)
            .map(|_| {
                (
                    q.next().unwrap().parse().unwrap(),
                    q.next().unwrap().parse().unwrap(),
                )
            })
            .collect();
        let r: Vec<String> = solve(n, v);
        assert_eq!(r, a);
    });
}
