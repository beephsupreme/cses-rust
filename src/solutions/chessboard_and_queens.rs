/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(c: usize, p: &mut [usize; 8], b: &Vec<Vec<bool>>) -> usize {
    if c == 8 {
        return 1_usize;
    }
    let mut t = 0;
    'outer: for r in 0..8 {
        if !b[r][c] {
            continue 'outer;
        }
        for (i, x) in p.iter().enumerate().take(c) {
            if *x == r {
                continue 'outer;
            }
            let (dx, dy) = (c as i8 - i as i8, r as i8 - *x as i8);
            if dx.abs() == dy.abs() {
                continue 'outer;
            }
        }
        p[c] = r;
        t += solve(c + 1, p, b);
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        let b: Vec<Vec<bool>> = vec![
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
        ];
        assert_eq!(solve(0, &mut [0usize; 8], &b), 92);
    }

    #[test]
    fn unit_02() {
        let b: Vec<Vec<bool>> = vec![
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, false, true, true, true],
        ];
        assert_eq!(solve(0, &mut [0usize; 8], &b), 74);
    }

    #[test]
    fn unit_03() {
        let b: Vec<Vec<bool>> = vec![
            vec![true, true, true, true, true, true, true, false],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, false, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, true],
            vec![true, true, true, true, true, true, false, false],
        ];
        assert_eq!(solve(0, &mut [0usize; 8], &b), 72);
    }

    #[test]
    fn unit_04() {
        let b: Vec<Vec<bool>> = vec![
            vec![true, true, true, false, false, true, true, true],
            vec![true, true, true, true, true, true, false, true],
            vec![false, true, true, true, true, false, true, true],
            vec![true, true, true, false, true, true, true, false],
            vec![true, true, true, true, false, false, true, false],
            vec![true, true, false, true, true, true, true, true],
            vec![true, true, true, true, true, true, true, false],
            vec![true, true, true, false, true, true, true, true],
        ];
        assert_eq!(solve(0, &mut [0usize; 8], &b), 11);
    }
}
