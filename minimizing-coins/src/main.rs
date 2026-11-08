//! https://cses.fi/problemset/task/1634

use std::{
    cmp::min,
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, target) = (parse(), parse());
    let coins: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut dp = vec![usize::MAX - 1; target + 1];
    dp[0] = 0;
    for i in 1..=target {
        for &c in &coins {
            if i >= c {
                dp[i] = min(dp[i], dp[i - c] + 1)
            }
        }
    }
    let res = *dp.last().unwrap();
    if res == usize::MAX - 1 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
