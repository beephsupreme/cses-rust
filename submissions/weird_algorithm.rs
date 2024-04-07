use std::fmt::Write;

fn main() {
    let mut n = String::new();
    std::io::stdin().read_line(&mut n).unwrap();
    let mut n: u64 = n.trim().parse().unwrap();
    let mut output = String::new();
    while n != 1 {
        write!(output, "{} ", n).unwrap();
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }
    write!(output, "1").unwrap();
    println!("{}", output);
}
