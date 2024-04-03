/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64) -> u64 {
    let mut result: u64 = 0;
    let mut i: u64 = 5;
    while n / i >= 1 {
        result += n / i;
        i *= 5;
    }
    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(395), 97);
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(871), 215);
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(239), 57);
    }

    #[test]
    fn unit_04() {
        assert_eq!(solve(535), 132);
    }
}
