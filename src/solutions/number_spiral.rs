/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64, v: Vec<(u64, u64)>) -> Vec<u64> {
    let mut output: Vec<u64> = Vec::new();
    for i in 0..n {
        let (i, j) = v[i as usize];
        let max = u64::max(i, j);
        let d = 1 + max * (max - 1);
        if max & 1 == 0 {
            output.push(d + i - j);
        } else {
            output.push(d + j - i);
        }
    }
    output
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(3, vec![(2, 3), (1, 1), (4, 2)]), vec![8, 1, 15]);
    }
}
