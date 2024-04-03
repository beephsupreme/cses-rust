/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u8) -> Vec<(u8, u8)> {
    let mut v: Vec<(u8, u8)> = Vec::new();
    recurse(n, 1, 3, &mut v);
    v
}
fn recurse(n: u8, a: u8, b: u8, v: &mut Vec<(u8, u8)>) {
    if n == 1 {
        v.push((a, b));
    } else {
        let c: u8 = 6 - (a + b);
        recurse(n - 1, a, c, v);
        v.push((a, b));
        recurse(n - 1, c, b, v)
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(1), vec![(1, 3)]);
    }
    #[test]
    fn unit_02() {
        assert_eq!(solve(2), vec![(1, 2), (1, 3), (2, 3)]);
    }
    #[test]
    fn unit_03() {
        assert_eq!(
            solve(3),
            vec![(1, 3), (1, 2), (3, 2), (1, 3), (2, 1), (2, 3), (1, 3)]
        );
    }
    #[test]
    fn unit_04() {
        assert_eq!(
            solve(4),
            vec![
                (1, 2),
                (1, 3),
                (2, 3),
                (1, 2),
                (3, 1),
                (3, 2),
                (1, 2),
                (1, 3),
                (2, 3),
                (2, 1),
                (3, 1),
                (2, 3),
                (1, 2),
                (1, 3),
                (2, 3)
            ]
        );
    }
}
