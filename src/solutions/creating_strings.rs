/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(s: String) -> Vec<String> {
    let mut v = s.chars().collect::<Vec<char>>();
    v.sort();
    let mut r = Vec::new();
    r.push(v.iter().collect());
    while next_permutation(&mut v) {
        r.push(v.iter().collect());
    }
    r
}
fn next_permutation(v: &mut [char]) -> bool {
    let n = v.len();
    for i in (0..n - 1).rev() {
        if v[i] < v[i + 1] {
            for j in (i + 1..n).rev() {
                if v[i] < v[j] {
                    v.swap(i, j);
                    break;
                }
            }
            v[i + 1..n].reverse();
            return true;
        }
    }
    false
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        let s: String = String::from("ABC");
        let r = solve(s);
        assert_eq!(r.len(), 6);
        assert_eq!(r, vec!["ABC", "ACB", "BAC", "BCA", "CAB", "CBA"]);
    }

    #[test]
    fn unit_02() {
        let s: String = String::from("A");
        let r = solve(s);
        assert_eq!(r.len(), 1);
        assert_eq!(r, vec!["A"]);
    }
}
