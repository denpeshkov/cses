//! https://cses.fi/problemset/task/1746

use std::{
    io::{stdin, Read},
    iter,
};

fn main() {
    const MOD: usize = 1000000007;

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, maxv) = (parse(), parse());
    let arr: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    // dp[i][j] - number of valid combinations for values in the array at indices [1..i],
    // such that the last of them, at index i, is equal to j
    //
    // dp has dummy(0) `0`th and `maxv+1`th element to eliminate `if`s
    let mut dp = vec![vec![0usize; maxv + 2]; n];
    if arr[0] == 0 {
        dp[0][1..maxv + 1].fill(1);
    } else {
        dp[0][arr[0]] = 1;
    }
    for i in 1..n {
        let v = arr[i];
        if v != 0 {
            dp[i][v] = (dp[i - 1][v - 1] + dp[i - 1][v] + dp[i - 1][v + 1]) % MOD;
        } else {
            for v in 1..=maxv {
                dp[i][v] = (dp[i - 1][v - 1] + dp[i - 1][v] + dp[i - 1][v + 1]) % MOD;
            }
        }
    }
    let sum = dp[n - 1].iter().fold(0, |acc, v| (acc + v) % MOD);
    println!("{sum}");
}
