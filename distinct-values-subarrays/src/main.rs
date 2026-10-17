//! https://cses.fi/problemset/task/3420

use std::{
    cmp,
    collections::HashMap,
    io::{Read, stdin},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let n: usize = toks.next().unwrap().parse().unwrap();
    let arr: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    // Sliding window
    let mut left_pos = 1;
    let mut last_pos: HashMap<usize, usize> = HashMap::new();

    let mut cnt = 0;
    for (i, x) in arr.iter().enumerate() {
        let i = i + 1;
        left_pos = cmp::max(left_pos, last_pos.get(x).unwrap_or(&0) + 1);
        cnt += i + 1 - left_pos;
        last_pos.insert(*x, i);
    }
    println!("{cnt}")
}
