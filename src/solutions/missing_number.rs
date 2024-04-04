/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MissingNumberError {
    #[error("n = {0} but expected 2 <= n <= 200_000")]
    InvalidInput(String),
    #[error("vector.len() = {0} but expected {1}")]
    InvalidVectorSize(String, String),
    #[error("vector sum ({0}) exceeds >= theoretical sum ({1})")]
    InvalidVectorValues(String, String),
}

/// Lower-bound for the input value.
const LBOUND: u64 = 2;

/// Upper-bound for the input value.
const UBOUND: u64 = 200_000;

/// # Missing Number
/// The problem is described in more detail at https://cses.fi/problemset/task/1083.
///
/// # Parameters
/// A positive number n and a vector of n-1 numbers.
///
/// # Returns
/// A Result which contains the missing number in the sequence or an error.
///
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(n).
pub fn solve(n: u64, v: Vec<u64>) -> Result<u64, MissingNumberError> {
    if !(LBOUND..=UBOUND).contains(&n) {
        return Err(MissingNumberError::InvalidInput(n.to_string()));
    }
    if v.len() != (n - 1) as usize {
        return Err(MissingNumberError::InvalidVectorSize(
            v.len().to_string(),
            (n - 1).to_string(),
        ));
    }
    let sum: u64 = n * (n + 1) / 2;
    let partial_sum: u64 = v.iter().sum::<u64>();
    if sum <= partial_sum {
        return Err(MissingNumberError::InvalidVectorValues(
            partial_sum.to_string(),
            sum.to_string(),
        ));
    }
    Ok(sum - partial_sum)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(5, vec![1, 2, 4, 5]).unwrap(), 3)
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(5, vec![1, 2, 3, 4]).unwrap(), 5)
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(5, vec![2, 3, 4, 5]).unwrap(), 1)
    }

    #[test]
    fn unit_04() {
        assert_eq!(solve(10, vec![1, 2, 4, 5, 6, 7, 8, 9, 10]).unwrap(), 3)
    }
}
