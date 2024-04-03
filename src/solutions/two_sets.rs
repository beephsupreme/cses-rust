/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        return None;
    } else {
        sum /= 2;
    }
    for i in (1..=n).rev() {
        if i <= sum {
            sum -= i;
            a.push(i);
        } else {
            b.push(i)
        }
    }
    Some((a, b))
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve(1), None);
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve(2), None);
    }

    #[test]
    fn unit_03() {
        let (u, v) = solve(3).unwrap();
        assert!(!(u.is_empty() ^ v.is_empty()));
        assert_eq!(u.len() + v.len(), 3);
        assert_eq!(u.iter().sum::<u64>() + v.iter().sum::<u64>(), 6);
    }

    #[test]
    fn unit_04() {
        let (u, v) = solve(8).unwrap();
        assert!(!(u.is_empty() ^ v.is_empty()));
        assert_eq!(u.len() + v.len(), 8);
        assert_eq!(u.iter().sum::<u64>() + v.iter().sum::<u64>(), 36);
    }
}
