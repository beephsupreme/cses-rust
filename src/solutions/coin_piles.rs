/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64, v: Vec<(u64, u64)>) -> Vec<String> {
    let mut r: Vec<String> = Vec::new();
    for i in 0..n {
        let (a, b) = v[i as usize];
        if a > 2 * b || b > 2 * a || (a + b) % 3 != 0 {
            r.push("NO".to_string());
        } else {
            r.push("YES".to_string());
        }
    }
    r
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(1, vec![(11, 4)]), vec!["NO".to_string()]);
    }

    #[test]
    fn unit_02() {
        assert_eq!(
            solve(3, vec![(2, 1), (2, 2), (3, 3)]),
            vec!["YES".to_string(), "NO".to_string(), "YES".to_string()]
        );
    }
    #[test]
    fn unit_03() {
        assert_eq!(solve(1, vec![(11, 4)]), vec!["NO".to_string()]);
    }

    #[test]
    fn unit_04() {
        assert_eq!(
            solve(3, vec![(2, 1), (2, 2), (3, 3)]),
            vec!["YES".to_string(), "NO".to_string(), "YES".to_string()]
        );
    }
}
