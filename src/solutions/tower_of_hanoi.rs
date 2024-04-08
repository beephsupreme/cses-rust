/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::ops::RangeInclusive;

use crate::utils::error::CsesError;

const VALID_RANGE: RangeInclusive<u8> = 1..=16;

/// The Tower Of Hanoi problem is described in detail at (https://cses.fi/problemset/task/2165).
/// It is a classic problem with a recursive solution. The problem is to
/// move a stack of n disks from rod 1 to rod 3 with the help of rod 2.
/// # Parameters
/// A positive number n such that 1 ≤ n ≤ 16
/// # Returns
/// A Result containing a vector of tuples containing the source rod and the destination rod for each move or an error.
/// # Performance
/// The time complexity of this solution is O(2^n).
/// The space complexity of this solution is O(n).
pub fn tower_of_hanoi_solution(n: u8) -> Result<Vec<(u8, u8)>, CsesError> {
    if !VALID_RANGE.contains(&n) {
        let msg = format!(
            "n is {} but expected {} ≤ n ≤ {}",
            n,
            VALID_RANGE.start(),
            VALID_RANGE.end()
        );
        return Err(CsesError::InvalidInput(msg));
    }
    let mut v: Vec<(u8, u8)> = Vec::new();
    recurse(n, 1, 3, &mut v);
    Ok(v)
}
fn recurse(n: u8, a: u8, b: u8, v: &mut Vec<(u8, u8)>) {
    if n == 1 {
        v.push((a, b));
    } else {
        let c: u8 = 6 - (a + b);
        recurse(n - 1, a, c, v);
        v.push((a, b));
        recurse(n - 1, c, b, v)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(tower_of_hanoi_solution(1).unwrap(), vec![(1, 3)]);
    }
    #[test]
    fn unit_02() {
        assert_eq!(
            tower_of_hanoi_solution(2).unwrap(),
            vec![(1, 2), (1, 3), (2, 3)]
        );
    }
    #[test]
    fn unit_03() {
        assert_eq!(
            tower_of_hanoi_solution(3).unwrap(),
            vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)]
        );
    }
    #[test]
    fn unit_04() {
        assert_eq!(
            tower_of_hanoi_solution(4).unwrap(),
            vec![
                (1, 2),
                (1, 3),
                (2, 3),
                (1, 2),
                (3, 1),
                (3, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (2, 1),
                (3, 1),
                (2, 3),
                (1, 2),
                (1, 3),
                (2, 3)
            ]
        );
    }
}
