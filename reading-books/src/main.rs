//! https://cses.fi/problemset/task/1631

use std::{
    cmp::{self},
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse();
    let mut times: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    times.sort_unstable();

    let res = cmp::max(times.iter().sum(), times.iter().max().unwrap() * 2);
    println!("{}", res);
}
