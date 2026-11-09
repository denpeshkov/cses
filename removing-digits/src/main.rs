//! https://cses.fi/problemset/task/1637

use std::{cmp::min, io::stdin};

fn main() {
    let num: usize = stdin().lines().next().unwrap().unwrap().parse().unwrap();

    let mut dp = vec![0; num + 1];
    for x in 1..=num {
        let mut y = x;
        dp[x] = 1_000_000_000; // INF
        while y > 0 {
            dp[x] = min(dp[x], dp[x - y % 10] + 1);
            y /= 10;
        }
    }
    println!("{}", dp.last().unwrap())
}
