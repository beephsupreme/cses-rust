/*
 * Copyright (c) 2024 Michael N. Rowsey
 * Licensed under the MIT license (http://opensource.org/licenses/MIT)
 * This file may not be copied, modified, or distributed except according to those terms.
 */

pub fn solve(input: String) -> u64 {
    let mut streak = 1u64;
    let mut longest = 1u64;
    let mut prev = b'@';
    input.as_bytes().iter().for_each(|&ch| match ch == prev {
        true => {
            streak += 1;
            longest = std::cmp::max(longest, streak);
        }
        false => {
            streak = 1;
            prev = ch;
        }
    });
    longest
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn unit_01() {
        assert_eq!(solve("ATTCGGGA".to_string()), 3);
    }

    #[test]
    fn unit_02() {
        assert_eq!(solve("AATTCGGGAAAA".to_string()), 4);
    }

    #[test]
    fn unit_03() {
        assert_eq!(solve("AAATTCGGGA".to_string()), 3);
    }

    #[test]
    fn unit_04() {
        assert_eq!(solve("AAAAATTCGGGA".to_string()), 5);
    }
}
