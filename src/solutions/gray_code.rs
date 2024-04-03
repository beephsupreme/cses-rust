/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(n: u64) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut h: u64;
    let mut p: u64;
    for i in 0..(1 << n) as u64 {
        let mut buffer = String::new();
        for j in 1..=n {
            h = 1 << j;
            p = 2 * h;
            if (i + h / 2) % p < h {
                buffer.push('0')
            } else {
                buffer.push('1')
            };
        }
        output.push(buffer);
    }
    output
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        let mut result = solve(2);
        let mut answer = vec!["00", "01", "11", "10"];
        result.sort();
        answer.sort();
        assert_eq!(result, answer);
    }

    #[test]
    fn unit_02() {
        let mut result = solve(3);
        let mut answer = vec!["000", "001", "011", "010", "110", "111", "101", "100"];
        result.sort();
        answer.sort();
        assert_eq!(result, answer);
    }

    #[test]
    fn unit_03() {
        let mut result = solve(4);
        let mut answer = vec![
            "0000", "0001", "0011", "0010", "0110", "0111", "0101", "0100", "1100", "1101", "1111",
            "1110", "1010", "1011", "1001", "1000",
        ];
        result.sort();
        answer.sort();
        assert_eq!(result, answer);
    }
}
