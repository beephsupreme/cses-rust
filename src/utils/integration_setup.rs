/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fs;

use log::info;

pub fn setup(p: &str) -> (Vec<String>, Vec<String>) {
    info!("integration_setup reading data/{} directory", p);
    let paths = fs::read_dir(format!("data/{}/", p)).unwrap();
    let mut questions: Vec<String> = Vec::new();
    let mut answers: Vec<String> = Vec::new();
    paths
        .map(|p| p.unwrap())
        .filter(|p| p.path().is_file())
        .for_each(|p| {
            let file = p.path().display().to_string();
            match file {
                f if f.contains("input") => questions.push(f),
                f if f.contains("output") => answers.push(f),
                _ => (),
            }
        });
    if questions.len() != answers.len() {
        panic!("questions and answers are not equal in length");
    }
    questions.sort();
    answers.sort();
    (questions, answers)
}
