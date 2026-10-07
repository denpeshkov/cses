//! https://cses.fi/problemset/task/1617

const MODULO: u64 = 10u64.pow(9) + 7;

use std::io::stdin;
fn main() {
    let mut n = String::new();
    stdin().read_line(&mut n).unwrap();
    let n: u32 = n.trim().parse().unwrap();
    println!("{}", pow(n));
}

// (a ⋅ b) mod m = ((a mod m) ⋅ (b mod m)) mod m
fn pow(n: u32) -> u64 {
    if n == 1 {
        return 2;
    }
    if n % 2 == 0 {
        let n = pow(n / 2);
        ((n % MODULO) * (n % MODULO)) % MODULO
    } else {
        let n = 2 * pow(n - 1);
        n % MODULO
    }
}
