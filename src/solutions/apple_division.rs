/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: usize, apples: Vec<usize>) -> usize {
    let mut min_diff = usize::MAX;
    for i in 0..1 << n {
        let mut sum1: usize = 0;
        let mut sum2: usize = 0;
        for j in 0..n {
            if i & 1 << j != 0 {
                sum1 += apples[j];
            } else {
                sum2 += apples[j];
            }
        }
        min_diff = min_diff.min((sum1 as isize - sum2 as isize).unsigned_abs());
    }
    min_diff
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        let v: Vec<usize> = vec![1, 2, 3, 4, 5];
        let n: usize = v.len();
        assert_eq!(solve(n, v), 1);
    }

    #[test]
    fn unit_02() {
        let v: Vec<usize> = vec![603, 324, 573, 493, 659, 521, 654, 70, 718, 257];
        let n: usize = v.len();
        assert_eq!(solve(n, v), 2);
    }
}
