//! https://cses.fi/problemset/task/1145/

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse();
    let nums: Vec<_> = (0..n).map(|_| parse()).collect();

    let mut dp = Vec::with_capacity(n);
    for v in nums {
        let l = dp.partition_point(|&x| x < v);
        if l == dp.len() {
            dp.push(v);
        } else {
            dp[l] = v;
        }
    }
    println!("{}", dp.len())
}
