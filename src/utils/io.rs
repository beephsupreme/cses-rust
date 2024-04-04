/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::{any, fs};
use std::io::{BufReader, Read};
use std::str::{FromStr, SplitAsciiWhitespace};

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IoError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("IO error!")]
    IoError,
    #[error("Parse error!")]
    ParseError,
}

/// Reads a line from standard input and parses it into a type T.
/// # Returns
/// The result of parsing the input into a type T or an error.
pub fn get_int<T: FromStr>() -> Result<T, IoError> {
    let mut input = String::with_capacity(16);
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err(IoError::IoError),
    }
    match input
        .trim()
        .parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(IoError::ParseError),
    }
}

/// Reads a line from standard input and parses it into a vector of type T.
/// # Returns
/// The result of parsing the input into a vector of type T or an error.
pub fn get_vector<T: FromStr>() -> Result<Vec<T>, IoError> {
    let mut input = String::with_capacity(256000);
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => (),
        Err(_) => return Err(IoError::IoError),
    }
    let mut v: Vec<T> = Vec::new();
    let tokens = input.split_ascii_whitespace();
    for token in tokens {
        match token.parse::<T>() {
            Ok(k) => v.push(k),
            Err(_) => return Err(IoError::ParseError),
        }
    }
    Ok(v)
}

pub fn get_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn string_to_vector<T: FromStr>(input: String) -> Vec<T> {
    input
        .split_ascii_whitespace()
        .map(|s| {
            s.parse::<T>()
                .ok()
                .unwrap_or_else(|| panic!("string_to_vector: parse fail"))
        })
        .collect()
}

/// Converts a vector of type T to a string.
/// # Arguments
/// * `v` - A vector of type T.
/// * `sep` - An optional separator string.
pub fn vector_to_string<T>(v: Vec<T>, sep: Option<&str>) -> String
    where
        T: ToString,
{
    match sep {
        Some(sep) => v
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(sep),
        None => v.into_iter().map(|e| e.to_string()).collect(),
    }
}

pub fn get_tuple_vector(n: u64) -> Vec<(u64, u64)> {
    (0..n)
        .map(|_| {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut it = input.split_ascii_whitespace();
            (
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

pub fn get_vector_vector_bool() -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();
    let mut b: Vec<bool> = Vec::new();
    for _ in 0..8 {
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let chars = input.chars();
        for c in chars {
            b.push(c == '.');
        }
        v.push(b.clone());
        b.clear();
    }
    v
}

pub fn file_to_vector_vector_bool(path: &str) -> Vec<Vec<bool>> {
    let mut v: Vec<Vec<bool>> = Vec::new();
    let mut b: Vec<bool> = Vec::new();
    let input: String = fs::read_to_string(path).unwrap();
    for line in input.lines() {
        let chars = line.chars();
        for c in chars {
            b.push(c == '.');
        }
        v.push(b.clone());
        b.clear();
    }
    v
}

pub fn file_to_string(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_string()
}

pub fn file_to_vector<T: FromStr>(path: &str) -> Vec<T> {
    fs::read_to_string(path)
        .unwrap()
        .split_ascii_whitespace()
        .map(|s| {
            s.parse::<T>()
                .ok()
                .unwrap_or_else(|| panic!("file_to_vector: parse fail"))
        })
        .collect()
}

pub fn file_to_int<T: FromStr>(path: &str) -> T {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .parse::<T>()
        .ok()
        .unwrap_or_else(|| panic!("file_to_int: parse fail"))
}

#[allow(clippy::expect_fun_call)]
pub fn get_token<T: FromStr>(tokens: &mut SplitAsciiWhitespace) -> T {
    if let Some(token) = tokens.next() {
        return token
            .parse::<T>()
            .ok()
            .expect(format!("\"{}\" -> {}", token, any::type_name::<T>()).as_str());
    } else {
        panic!("get_token: expected Some, got None");
    }
}

pub fn load_tokens(b: &mut String) -> SplitAsciiWhitespace {
    let mut reader = BufReader::new(std::io::stdin());
    reader
        .read_to_string(b)
        .expect("load_tokens: failed to read from stdin");
    b.split_ascii_whitespace()
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_string_to_vector() {
    //     let s = String::from("1 2 3 4 5");
    //     let v: Vec<u64> = string_to_vector(s);
    //     assert_eq!(v, vec![1, 2, 3, 4, 5]);
    // }
    //
    // #[test]
    // fn test_vector_to_string() {
    //     let v = vec!['a', 'b', 'c', 'd', 'e'];
    //     assert_eq!("a, b, c, d, e".to_string(), vector_to_string(v, Some(", ")));
    // }
    //
    // #[test]
    // fn test_get_vector() {
    //     let mut input = String::new();
    //     std::io::stdin().read_line(&mut input).unwrap();
    //     let expected = vec![1, 2, 3, 4, 5];
    //     assert_eq!(get_vector::<u64>().unwrap(), expected);
    // }
}