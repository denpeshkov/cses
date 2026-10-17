//! https://cses.fi/problemset/task/3421

use std::{
    collections::HashMap,
    io::{Read, stdin},
};

const MODULO: usize = 10usize.pow(9) + 7;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let n: usize = toks.next().unwrap().parse().unwrap();
    let arr: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    let mut cnt = 1;
    let mut unique: HashMap<usize, usize> = HashMap::new();
    for x in arr {
        *unique.entry(x).or_default() += 1;
    }
    for v in unique.values() {
        cnt = (cnt % MODULO * (v + 1) % MODULO) % MODULO;
    }
    cnt -= 1;
    println!("{cnt}")
}
