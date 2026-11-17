//! https://cses.fi/problemset/task/1093

use std::io::stdin;

fn main() {
    const MOD: usize = 1000000007;
    const INV2: usize = 500000004;

    let n: usize = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    let sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        println!("0");
        return;
    }
    let sum = sum / 2;

    // dp[i][j] - number of ways to have sum j using elements [0..i].
    let mut dp = vec![vec![0; sum + 1]; n + 1];
    for v in &mut dp {
        v[0] = 1;
    }
    for i in 1..=n {
        for j in 1..=sum {
            dp[i][j] = (dp[i][j] + dp[i - 1][j]) % MOD;
            if j >= i {
                dp[i][j] = (dp[i][j] + dp[i - 1][j - i]) % MOD;
            }
        }
    }
    println!("{}", dp[n][sum] * INV2 % MOD)
}
