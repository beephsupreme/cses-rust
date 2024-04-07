/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::ops::RangeInclusive;

use crate::utils::error::CsesError;

const VALID_RANGE: RangeInclusive<i64> = 1..=10_000;

pub fn solve(n: i64) -> Result<Vec<i64>, CsesError> {
    if !VALID_RANGE.contains(&n) {
        let msg = format!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            VALID_RANGE.start(),
            VALID_RANGE.end()
        );
        return Err(CsesError::InvalidInput(msg));
    }
    let mut v: Vec<i64> = Vec::new();
    for i in 1..=n {
        v.push(i * i * (i * i - 1) / 2 - 4 * (i - 1) * (i - 2));
    }
    Ok(v)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn two_knights_0() {
        let expected = CsesError::InvalidInput("n is 0 but expected 1 ≤ n ≤ 10000".to_string());
        assert_eq!(solve(0), Err(expected));
    }

    #[test]
    fn two_knights_10001() {
        let expected = CsesError::InvalidInput("n is 10001 but expected 1 ≤ n ≤ 10000".to_string());
        assert_eq!(solve(10001), Err(expected));
    }

    #[test]
    fn two_knights_1() {
        assert_eq!(solve(1).unwrap(), [0]);
    }

    #[test]
    fn two_knights_8() {
        assert_eq!(solve(8).unwrap(), [0, 6, 28, 96, 252, 550, 1056, 1848]);
    }
}
