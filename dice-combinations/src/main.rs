//! https://cses.fi/problemset/task/1633

use std::io::{stdin, Read};

fn main() {
    const MOD: usize = 1_000_000_000 + 7;
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let n = input.trim().parse().unwrap();

    let mut dp = vec![0usize; n + 1];
    dp[0] = 1;
    for i in 1..=n {
        for j in 1..=6 {
            if i >= j {
                dp[i] = (dp[i] % MOD + dp[i - j] % MOD) % MOD;
            }
        }
    }
    println!("{}", dp[n])
}
