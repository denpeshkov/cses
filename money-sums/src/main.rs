//! https://cses.fi/problemset/task/1745

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse();
    let coins: Vec<_> = (0..n).map(|_| parse()).collect();
    let coins_sum = coins.iter().sum();

    // dp[i][j] - is it possible to to have sum j using subset of coins[..i].
    let mut dp = vec![vec![false; coins_sum + 1]; n + 1];
    for v in &mut dp {
        v[0] = true; // Always possible to have empty sum, by choosing no coins.
    }
    for i in 1..=n {
        for j in 1..=coins_sum {
            // Use the coin.
            if j >= coins[i - 1] {
                dp[i][j] |= dp[i - 1][j - coins[i - 1]];
            }
            // Dont' use the coin.
            dp[i][j] |= dp[i - 1][j];
        }
    }
    let cnt: usize = dp[n].iter().map(|&b| if b { 1 } else { 0 }).sum();
    println!("{}", cnt - 1); // Skip 0.
    for (i, &v) in dp[n].iter().skip(1).enumerate() {
        if v {
            print!("{} ", i + 1);
        }
    }
}
