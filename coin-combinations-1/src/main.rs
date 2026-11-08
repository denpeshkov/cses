//! https://cses.fi/problemset/task/1635

use std::{
    io::{stdin, Read},
    iter,
};

fn main() {
    const MOD: usize = 1_000_000_000 + 7;

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, target) = (parse(), parse());
    let coins: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut dp = vec![0; target + 1];
    dp[0] = 1;
    for i in 1..=target {
        for &c in &coins {
            if i >= c {
                dp[i] = (dp[i - c] + dp[i]) % MOD;
            }
        }
    }
    println!("{}", dp.last().unwrap())
}
