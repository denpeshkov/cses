//! https://cses.fi/problemset/task/1653

use std::{
    cmp::min,
    io::{stdin, Read},
    vec,
};

fn main() {
    const INF: usize = 1000;

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, x) = (parse(), parse());
    let weights: Vec<_> = (0..n).map(|_| parse()).collect();

    // dp[s] = (a,b):
    // a - the minimum number of rides for a subset `s`.
    // b - the minimum weight of the last ride in a solution where the number of rides is minimum.
    let mut dp = vec![(0usize, 0usize); 1 << n];
    dp[0] = (1, 0);

    // For each subset of people.
    for s in 1..(1 << n) {
        dp[s] = (INF, 0);
        // For each new person.
        for (i, &w) in weights.iter().enumerate() {
            if s & (1 << i) == 0 {
                // New person doesn't belong to set `s`.
                continue;
            }
            // Original subset `s` without the new person.
            let s2 = s ^ (1 << i);
            if dp[s2].1 + weights[i] <= x {
                // Put the new person into the existing last elevator ride.
                dp[s] = min(dp[s], (dp[s2].0, dp[s2].1 + w));
            } else {
                // Put the new person into a new ride.
                dp[s] = min(dp[s], (dp[s2].0 + 1, w));
            }
        }
    }
    println!("{}", dp[(1 << n) - 1].0);
}
