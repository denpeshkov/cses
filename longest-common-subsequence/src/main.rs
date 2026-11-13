//! https://cses.fi/problemset/task/3403

use std::{
    cmp::max,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, m) = (parse(), parse());
    let s1: Vec<_> = (0..n).map(|_| parse()).collect();
    let s2: Vec<_> = (0..m).map(|_| parse()).collect();

    let mut dp = vec![vec![0usize; m + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=m {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }

    println!("{}", dp[n][m]);

    fn print_lcs(s1: &Vec<usize>, s2: &Vec<usize>, dp: &Vec<Vec<usize>>, i: usize, j: usize) {
        if i == 0 || j == 0 {
            return;
        }
        if s1[i - 1] == s2[j - 1] {
            print_lcs(s1, s2, dp, i - 1, j - 1);
            print!("{} ", s1[i - 1]);
            return;
        };
        if dp[i][j] == dp[i - 1][j] {
            print_lcs(s1, s2, dp, i - 1, j);
        } else {
            print_lcs(s1, s2, dp, i, j - 1);
        }
    }
    print_lcs(&s1, &s2, &dp, n, m);
}
