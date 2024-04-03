/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(vector: Vec<u64>) -> u64 {
    let mut previous = vector[0];
    let mut current: u64 = 0;
    for v in vector {
        if v < previous {
            current += previous - v;
        } else {
            previous = v;
        }
    }
    current
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(vec![3, 2, 5, 1, 7]), 5)
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(vec![6, 10, 4, 10, 2, 8, 9, 2, 7, 7]), 31)
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0)
    }

    #[test]
    fn unit_04() {
        assert_eq!(
            solve(vec![1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1]),
            8999999991
        )
    }
}
