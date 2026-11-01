//! https://cses.fi/problemset/task/1620

use std::{
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse::<usize>().unwrap();

    let (n, t) = (parse(), parse());
    let times: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let (mut l, mut r) = (0, times.iter().min().unwrap() * t);
    while l < r {
        let m = (r - l) / 2 + l;
        let count = times.iter().cloned().fold(0, |acc, v| acc + m / v);
        if count >= t {
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{l}");
}
