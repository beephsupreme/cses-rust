/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

const LBOUND: u64 = 1;
const UBOUND: u64 = 1_000_000;

/// # Weird Algorithm
/// Takes as input a positive integer n. If n is even, the algorithm divides it by two, and
/// if n is odd, the algorithm multiplies it by three and adds one. The algorithm repeats
/// until n is one.
///
/// # Panics
/// If n is not within the range 1 <= n <= 1e6.
///
/// # Performance
/// The time complexity of this solution is O(n).
/// The space complexity of this solution is O(1).
pub fn solve(mut n: u64) -> Vec<u64> {
    if !(LBOUND..=UBOUND).contains(&n) {
        panic!("n = {}: n should be 1 <= n <= 1e6", n);
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
    v
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        let input: u64 = 1;
        let expected: Vec<u64> = vec![1];
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn unit_02() {
        let input: u64 = 2;
        let expected: Vec<u64> = vec![2, 1];
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn unit_03() {
        let input: u64 = 3;
        let expected: Vec<u64> = vec![3, 10, 5, 16, 8, 4, 2, 1];
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn unit_04() {
        let input: u64 = 4;
        let expected: Vec<u64> = vec![4, 2, 1];
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn unit_05() {
        let input: u64 = 15;
        let expected: Vec<u64> = vec![
            15, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1,
        ];
        assert_eq!(solve(input), expected);
    }

    #[test]
    fn unit_06() {
        let input: u64 = 23;
        let expected: Vec<u64> = vec![23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8, 4, 2, 1];
        assert_eq!(solve(input), expected);
    }
}
