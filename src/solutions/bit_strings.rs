/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64) -> u64 {
    let modulo = 1_000_000_007;
    let mut result = 1;
    for _ in 0..n {
        result = (result * 2) % modulo;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(7), 128);
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(15), 32768);
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(27), 134217728);
    }
}

#[test]
fn unit_04() {
    assert_eq!(solve(255), 396422633);
}
