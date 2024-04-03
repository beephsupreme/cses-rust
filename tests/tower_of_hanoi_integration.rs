/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use log::info;

use cses::solutions::tower_of_hanoi::solve;
use cses::utils::integration_setup::setup;
use cses::utils::io::{file_to_int, file_to_string};

#[cfg(test)]
#[test]
fn integration_01() {
    env_logger::init();
    let (questions, answers) = setup("tower_of_hanoi");
    (0..questions.len()).for_each(|i| {
        info!("{}: {}", i + 1, questions[i]);
        let n: u8 = file_to_int(&questions[i]);
        let answer: String = file_to_string(&answers[i]);
        let mut answer = answer.split_ascii_whitespace();
        let len: usize = answer.next().unwrap().parse().unwrap();
        let mut a: Vec<(u8, u8)> = Vec::new();
        for _ in 0..len {
            a.push((
                answer.next().unwrap().parse().unwrap(),
                answer.next().unwrap().parse().unwrap(),
            ));
        }
        let r: Vec<(u8, u8)> = solve(n);
        assert_eq!(r, a);
    });
}
