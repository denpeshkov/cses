//! https://cses.fi/problemset/task/1638

use std::io::stdin;

fn main() {
    const MOD: usize = 1000000007;
    let mut lines = stdin().lines();

    let n = lines.next().unwrap().unwrap().parse().unwrap();
    let grid: Vec<_> = lines.take(n).map(|l| l.unwrap()).collect();

    // Top-left cell is a trap.
    if grid[0][0..=0] == *"*" {
        println!("0");
        return;
    }

    let mut dp = vec![vec![0usize; n]; n];
    dp[0][0] = 1;
    for r in 0..n {
        for c in 0..n {
            if grid[r][c..=c] == *"*" {
                continue;
            }
            if r >= 1 {
                dp[r][c] += dp[r - 1][c]
            }
            if c >= 1 {
                dp[r][c] += dp[r][c - 1];
            }
            dp[r][c] %= MOD;
        }
    }
    println!("{}", dp[n - 1][n - 1])
}
