/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use log::info;

use cses::solutions::tower_of_hanoi::tower_of_hanoi_solution;
use cses::utils::integration_setup::get_test_filenames;
use cses::utils::io::{get_token, load_all_tokens};

#[cfg(test)]
#[test]
fn tower_of_hanoi_integration() -> Result<()> {
    env_logger::init();
    let (questions, answers) = get_test_filenames("tower_of_hanoi");
    for i in 0..questions.len() {
        info!("{}: {}", i + 1, questions[i]);
        let mut q_reader = BufReader::new(File::open(&questions[i])?);
        let mut a_reader = BufReader::new(File::open(&answers[i])?);
        let mut q_buffer: String = String::new();
        let mut a_buffer: String = String::new();
        let mut q_tokens = load_all_tokens(&mut q_reader, &mut q_buffer)?;
        let mut a_tokens = load_all_tokens(&mut a_reader, &mut a_buffer)?;
        let n: u8 = get_token(&mut q_tokens)?;
        let a: usize = get_token(&mut a_tokens)?;
        let mut v: Vec<(u8, u8)> = Vec::new();
        for _ in 0..a {
            v.push((get_token(&mut a_tokens)?, get_token(&mut a_tokens)?));
        }
        let r: Vec<(u8, u8)> = tower_of_hanoi_solution(n)?;
        let mut r1v: Vec<u8> = Vec::new();
        let mut r2v: Vec<u8> = Vec::new();
        for (r1, r2) in r.iter() {
            r1v.push(*r1);
            r2v.push(*r2);
        }
        let mut v1v: Vec<u8> = Vec::new();
        let mut v2v: Vec<u8> = Vec::new();
        for (v1, v2) in r.iter() {
            v1v.push(*v1);
            v2v.push(*v2);
        }
        assert_eq!(r1v, v1v);
        assert_eq!(r2v, v2v);
    }
    Ok(())
}
