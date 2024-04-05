/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */
use std::{any, fs};
use std::io::{BufReader, Read, Write};
use std::str::{FromStr, SplitAsciiWhitespace};

use anyhow::{bail, Context, Result};
use crate::utils::error::CsesError;


pub fn get_bytes<R>(mut reader: R) -> Result<Vec<u8>>
    where R: std::io::BufRead {
    let mut input = String::new();
    let mut output: Vec<u8> = Vec::new();
    reader.read_line(&mut input)?;
    write!(output, "{}", input)?;
    Ok(output)
}

/// # get_int()
/// Reads a line from standard input and parses it into a type T.
/// # Returns
/// The result of parsing the input into a type T or an error.
pub fn get_int<T: FromStr>(v: Vec<u8>) -> Result<T> {
    let s = String::from_utf8(v)?;
    match s.trim().parse::<T>() {
        Ok(r) => Ok(r),
        Err(_) => bail!(CsesError::ParseError(format!("cses::utils::io::get_int({:?})", s))),
    }
}

/// # get_vector()
/// Reads a line from standard input and parses it into a vector of type T.
/// # Returns
/// The result of parsing the input into a vector of type T or an error.
pub fn get_vector<T: FromStr>() -> Result<Vec<T>> {
    let mut input = String::with_capacity(4096);
    std::io::stdin().read_line(&mut input).context("cses::utils::io::get_vector: Could not read from stdin")?;
    let mut v: Vec<T> = Vec::new();
    let tokens = input.split_ascii_whitespace();
    for token in tokens {
        match token.parse::<T>() {
            Ok(k) => v.push(k),
            Err(_) => bail!(CsesError::ParseError(format!("cses::utils::io::get_vector: {}", token.to_string()))),
        }
    }
    Ok(v)
}

/// # get_string()
/// Reads a line from standard input and returns a string.
/// # Returns
/// A Result contining a trimmed string or an error.
pub fn get_string() -> Result<String> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).context("cses::utils::io::get_string: Could not read from stdin")?;
    Ok(input.trim().to_string())
}

/// # string_to_vector()
/// Converts a string into a vector of type T.
/// # Arguments
/// * `input` - A string to be converted into a vector of type T.
/// # Returns
/// A Result containing a vector of type T or an error.
pub fn string_to_vector<T: FromStr>(input: String) -> Result<Vec<T>> {
    let tokens = input.split_ascii_whitespace();
    let mut v: Vec<T> = Vec::new();
    for token in tokens {
        match token.parse::<T>() {
            Ok(k) => v.push(k),
            Err(_) => bail!(CsesError::ParseError(format!("cses::utils::io::string_to_vector: {}", token.to_string()))),
        }
    }
    Ok(v)
}

/// # vector_to_string()
/// Converts a vector of type T to a string.
/// # Arguments
/// * `v` - A vector of type T.
/// * `sep` - An optional separator string.
/// # Returns
/// A string containing the elements of the vector separated by the separator.
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

/// # get_tuple_vector()
/// Reads n lines from standard input and parses them into a vector of tuples of type (u64, u64).
/// # Arguments
/// * `n` - The number of lines to read.
/// # Returns
/// A Result containing a vector of tuples of type (u64, u64) or an error.
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

/// # get_vector_vector_bool()
/// Reads 8 lines from standard input and parses them into a vector of vectors of bool.
/// # Returns
/// A Result containing a vector of vectors of bool or an error.
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

/// # file_to_vector_vector_bool()
/// Reads a file and parses it into a vector of vectors of bool.
/// # Arguments
/// * `path` - A string representing the path to the file.
/// # Returns
/// A Result containing a vector of vectors of bool or an error.
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

/// # file_to_string()
/// Reads a file and returns a string.
/// # Arguments
/// * `path` - A string representing the path to the file.
/// # Returns
/// A Result containing a string or an error.
pub fn file_to_string(path: &str) -> String {
    fs::read_to_string(path).unwrap().trim().to_string()
}

/// # file_to_vector()
/// Reads a file and parses it into a vector of type T.
/// # Arguments
/// * `path` - A string representing the path to the file.
/// # Returns
/// A Result containing a vector of type T or an error.
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

/// # file_to_int()
/// Reads a file and parses it into a type T.
/// # Arguments
/// * `path` - A string representing the path to the file.
/// # Returns
/// A Result containing a type T or an error.
pub fn file_to_int<T: FromStr>(path: &str) -> T {
    fs::read_to_string(path)
        .unwrap()
        .trim()
        .parse::<T>()
        .ok()
        .unwrap_or_else(|| panic!("file_to_int: parse fail"))
}

/// # get_token()
/// Parses a token into a type T.
/// # Arguments
/// * `tokens` - A mutable reference to a SplitAsciiWhitespace.
/// # Returns
/// A Result containing a type T or an error.
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

/// # load_tokens()
/// Reads from standard input and returns a SplitAsciiWhitespace.
/// # Arguments
/// * `b` - A mutable reference to a string.
/// # Returns
/// A Result containing a SplitAsciiWhitespace or an error.
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