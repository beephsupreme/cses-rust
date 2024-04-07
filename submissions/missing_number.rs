use std::io::{BufReader, Read};

fn main() {
    let mut reader = BufReader::new(std::io::stdin());
    let mut buffer: String = String::new();
    reader.read_to_string(&mut buffer).unwrap();
    let mut tokens = buffer.split_ascii_whitespace();
    let mut n: u64 = 0;
    if let Some(token) = tokens.next() {
        match token.parse::<u64>() {
            Ok(r) => n = r,
            Err(_) => panic!("ParseError"),
        }
    }
    let mut sum: u64 = n * (n + 1) / 2;
    for token in tokens.by_ref() {
        match token.parse::<u64>() {
            Ok(r) => sum -= r,
            Err(_) => panic!("ParseError"),
        }
    }
    println!("{}", sum);
}
