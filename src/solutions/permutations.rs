/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fmt::Write;

pub fn solve(n: u64) -> String {
    let mut output = String::with_capacity(6_888_896);
    match n {
        0 => unreachable!(),
        1 => write!(output, "1").unwrap(),
        2 | 3 => write!(output, "NO SOLUTION").unwrap(),
        _ => {
            let half = n / 2;
            for i in 0..half {
                write!(output, "{} ", 2 * i + 2).unwrap();
            }
            for i in 0..n - half {
                write!(output, "{} ", 2 * i + 1).unwrap();
            }
        }
    }
    output.trim().to_string()
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(1), "1");
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(2), "NO SOLUTION");
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve(3), "NO SOLUTION");
    }

    #[test]
    fn unit_04() {
        assert_eq!(solve(7), "2 4 6 1 3 5 7");
    }
}
