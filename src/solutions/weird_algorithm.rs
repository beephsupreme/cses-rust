/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::ops::RangeInclusive;

use crate::utils::error::CsesError;

const VALID_RANGE: RangeInclusive<u64> = 1..=1_000_000;

/// The problem is described in detail at (https://cses.fi/problemset/task/1068).
/// If the number is even, divide it by two. If it is odd, multiply it by three and add one. Repeat this process until the number is one.
/// # Parameters
/// A positive number n such that 1 ≤ n ≤ 1e^6
/// # Returns
/// A vector of numbers generated by the algorithm or an error.
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(1) because the vector is pre-allocated. Otherwise, it would be O(n).
pub fn solve(mut n: u64) -> Result<Vec<u64>, CsesError> {
    if !(VALID_RANGE).contains(&n) {
        let msg = format!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            VALID_RANGE.start(),
            VALID_RANGE.end()
        );
        return Err(CsesError::InvalidInput(msg));
    }
    let mut v: Vec<u64> = Vec::with_capacity(4096);
    loop {
        v.push(n);
        if n == 1 {
            break;
        }
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    Ok(v)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn weird_algorithm_unit_0() {
        let n: u64 = 0;
        let expected: CsesError =
            CsesError::InvalidInput("n is 0 but expected 1 ≤ n ≤ 1000000".to_string());
        let result = solve(n);
        assert_eq!(result, Err(expected));
    }

    #[test]
    fn weird_algorithm_unit_1_000_001() {
        let n: u64 = 1_000_001;
        let expected: CsesError =
            CsesError::InvalidInput("n is 1000001 but expected 1 ≤ n ≤ 1000000".to_string());
        let result = solve(n);
        assert_eq!(result, Err(expected));
    }

    #[test]
    fn weird_algorithm_unit_23() {
        let n: u64 = 23;
        let expected: Vec<u64> = vec![23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1];
        let result = solve(n);
        assert_eq!(result.unwrap(), expected);
    }
}
