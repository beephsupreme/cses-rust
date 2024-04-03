/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: i64) -> Vec<i64> {
    let mut v: Vec<i64> = Vec::new();
    for i in 1..=n {
        v.push(i * i * (i * i - 1) / 2 - 4 * (i - 1) * (i - 2));
    }
    v
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(1), [0]);
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(2), [0, 6]);
    }
}
