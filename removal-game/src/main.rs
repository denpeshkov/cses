//! https://cses.fi/problemset/task/1097

use std::{
    cmp::max,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse() as usize;
    let list: Vec<i64> = (0..n).map(|_| parse()).collect();

    // dp[i][j][p] - max total value I can take starting from list[i..j].
    let mut dp = vec![vec![0i64; n]; n];

    let mut pref_sum = vec![0i64; n + 1];
    for (i, v) in list.iter().enumerate() {
        pref_sum[i + 1] = pref_sum[i] + v;
    }

    for i in (0..n).rev() {
        for j in i..n {
            if i == j {
                dp[i][j] = list[i];
                continue;
            }
            dp[i][j] = max(
                list[i] + (pref_sum[j + 1] - pref_sum[i + 1]) - dp[i + 1][j],
                list[j] + (pref_sum[j] - pref_sum[i]) - dp[i][j - 1],
            );
        }
    }
    println!("{}", dp[0][n - 1])
}
