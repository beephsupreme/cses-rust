/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::collections::HashSet;
use std::ops::RangeInclusive;

use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[non_exhaustive]
pub enum MissingNumberError {
    #[error("n = {0} but expected 2 <= n <= 200_000")]
    OutOfRange(u64),
    #[error("vector.len() = {0} but expected {1}")]
    InvalidVectorLength(usize, u64),
    #[error("vector element cannot exceed {0}")]
    MaximumVectorElement(u64),
    #[error("vector element cannot be 0")]
    MinimumVectorElement,
    #[error("vector only has {0} unique  values, {1}) expected")]
    DuplicateValues(usize, u64),
}

const VALID_RANGE: RangeInclusive<u64> = 2..=200_000;

/// The problem is described in detail at https://cses.fi/problemset/task/1083.
/// Given a sequence of n - 1 distinct integers in the range 1, 2, ..., n, find the missing number.
/// #
/// # Parameters
/// * n: A positive number n such that 2 <= n <= 200_000.
/// * v: A vector of n - 1 distinct elements such that 1 <= v[i] <= n.
/// #
/// # Returns
/// A Result which contains the missing number in the sequence or an error.  
/// #
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(n).
/// #
/// # Examples
/// ```
/// use cses::solutions::missing_number::solve;
/// let n: u64 = 5;
/// let v: Vec<u64> = vec![1, 2, 4, 5];
/// let result: u64 = solve(n, v).unwrap();
/// assert_eq!(result, 3);
/// ```
pub fn solve(n: u64, v: Vec<u64>) -> Result<u64, MissingNumberError> {
    if !VALID_RANGE.contains(&n) {
        return Err(MissingNumberError::OutOfRange(n));
    }
    if v.len() != (n - 1) as usize {
        return Err(MissingNumberError::InvalidVectorLength(v.len(), n - 1));
    }
    let vmax = *v.iter().max().unwrap();
    if vmax > n {
        return Err(MissingNumberError::MaximumVectorElement(n));
    }
    let vmin = *v.iter().min().unwrap();
    if vmin == 0 {
        return Err(MissingNumberError::MinimumVectorElement);
    }
    let mut unique: HashSet<u64> = HashSet::new();
    v.iter().all(|x| unique.insert(*x));
    if unique.len() != (n - 1) as usize {
        return Err(MissingNumberError::DuplicateValues(unique.len(), n - 1));
    }
    let sum: u64 = n * (n + 1) / 2;
    let partial_sum: u64 = v.iter().sum::<u64>();
    Ok(sum - partial_sum)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn weird_algorithm_0() {
        let expected: MissingNumberError = MissingNumberError::MinimumVectorElement;
        assert_eq!(solve(5, vec![0, 1, 2, 3]), Err(expected));
    }
    #[test]
    fn weird_algorithm_1() {
        assert_eq!(solve(5, vec![2, 3, 4, 5]).unwrap(), 1)
    }

    #[test]
    fn weird_algorithm_3() {
        assert_eq!(solve(5, vec![1, 2, 4, 5]).unwrap(), 3)
    }

    #[test]
    fn weird_algorithm_5() {
        assert_eq!(solve(5, vec![1, 2, 3, 4]).unwrap(), 5)
    }

    #[test]
    fn weird_algorithm_6() {
        let n: u64 = 5;
        let expected: MissingNumberError = MissingNumberError::MaximumVectorElement(n);
        assert_eq!(solve(n, vec![1, 2, 3, 6]), Err(expected));
    }

    #[test]
    fn weird_algorithm_200001() {
        let expected: MissingNumberError = MissingNumberError::OutOfRange(200_001);
        let mut v: Vec<u64> = Vec::new();
        (1..200_001).for_each(|i| v.push(i as u64));
        assert_eq!(solve(200_001, v), Err(expected));
    }
}
