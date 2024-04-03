/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64, v: Vec<u64>) -> u64 {
    let r: u64 = n * (n + 1) / 2 - v.iter().sum::<u64>();
    r
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(5, vec![1, 2, 4, 5]), 3)
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(5, vec![1, 2, 3, 4]), 5)
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(5, vec![2, 3, 4, 5]), 1)
    }

    #[test]
    fn unit_04() {
        assert_eq!(solve(10, vec![1, 2, 4, 5, 6, 7, 8, 9, 10]), 3)
    }
}
