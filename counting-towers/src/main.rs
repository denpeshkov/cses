//! https://cses.fi/problemset/task/2413

use std::io::stdin;

fn main() {
    const MOD: usize = 1000000007;
    let mut lines = stdin().lines();
    let mut parse = || lines.next().unwrap().unwrap().parse().unwrap();

    let n = parse();
    let towers: Vec<_> = (0..n).map(|_| parse()).collect();

    for th in towers {
        let mut dp = vec![[0usize; 2]; th];
        dp[0][0] = 1;
        dp[0][1] = 1;
        for h in 1..th {
            dp[h][0] = (4 * dp[h - 1][0] + dp[h - 1][1]) % MOD;
            dp[h][1] = (2 * dp[h - 1][1] + dp[h - 1][0]) % MOD;
        }
        println!("{}", (dp[th - 1][0] + dp[th - 1][1]) % MOD)
    }
}
