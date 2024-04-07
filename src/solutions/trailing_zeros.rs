/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::ops::RangeInclusive;

use anyhow::Result;

use crate::utils::error::CsesError;

const VALID_RANGE: RangeInclusive<u64> = 1..=1_000_000_000;

/// Solution to (https://cses.fi/problemset/task/1618)
/// Calculate the number of trailing zeros in the factorial of a number.
/// # Parameters
/// * n: u64 - The number to calculate the factorial of.
/// # Returns
/// A u64 representing the number of trailing zeros in the factorial of n.
pub fn solve(n: u64) -> Result<u64, CsesError> {
    if !VALID_RANGE.contains(&n) {
        let msg = format!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            VALID_RANGE.start(),
            VALID_RANGE.end()
        );
        return Err(CsesError::InvalidInput(msg));
    }
    let mut result: u64 = 0;
    let mut i: u64 = 5;
    while n / i >= 1 {
        result += n / i;
        i *= 5;
    }
    Ok(result)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn trailing_zeros_unit_0() {
        let expected: CsesError =
            CsesError::InvalidInput("n is 0 but expected 1 ≤ n ≤ 1000000000".to_string());
        assert_eq!(solve(0), Err(expected));
    }

    #[test]
    fn trailing_zeros_unit_1000000001() {
        let expected: CsesError =
            CsesError::InvalidInput("n is 1000000001 but expected 1 ≤ n ≤ 1000000000".to_string());
        assert_eq!(solve(1000000001), Err(expected));
    }
    #[test]
    fn trailing_zeros_unit_395() {
        assert_eq!(solve(395).unwrap(), 97);
    }

    #[test]
    fn trailing_zeros_unit_871() {
        assert_eq!(solve(871).unwrap(), 215);
    }

    #[test]
    fn trailing_zeros_unit_239() {
        assert_eq!(solve(239).unwrap(), 57);
    }

    #[test]
    fn trailing_zeros_unit_535() {
        assert_eq!(solve(535).unwrap(), 132);
    }
}
