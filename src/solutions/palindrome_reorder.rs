/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

use std::fmt::Write;

pub fn solve(input: String) -> Option<String> {
    _ = input.trim();
    let mut output = String::new();
    let mut front = String::new();
    let mut freq: [u64; 26] = [0; 26];
    let mut middle: Option<u8> = None;
    for c in input.chars() {
        freq[(c as u8 - b'A') as usize] += 1;
    }
    for (i, f) in freq.iter().enumerate() {
        if f % 2 == 1 {
            if middle.is_some() {
                return None;
            }
            middle = Some(i as u8 + b'A');
        }
    }
    for (i, f) in freq.iter().enumerate() {
        for _ in 0..f / 2 {
            front.push((i as u8 + b'A') as char);
        }
    }
    let back: String = front.chars().rev().collect();
    write!(output, "{}", front).unwrap();
    if middle.is_some() {
        write!(output, "{}", middle.unwrap() as char).unwrap();
    }
    write!(output, "{}", back).unwrap();
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(
            solve("AAAAAAAAAA".to_string()),
            Some("AAAAAAAAAA".to_string())
        );
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve("ABABABABAB".to_string()), None);
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve("MWNYFUIRUX".to_string()), None);
    }

    #[test]
    fn unit_04() {
        assert_eq!(
            solve("AAAACACBA".to_string()),
            Some("AAACBCAAA".to_string())
        );
    }
}
