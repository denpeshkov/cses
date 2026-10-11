//! https://cses.fi/problemset/task/1621

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut toks = input.split_whitespace();
    let n: usize = toks.next().unwrap().parse().unwrap();
    let mut nums: Vec<i32> = toks.take(n).map(|s| s.parse().unwrap()).collect();

    nums.sort_unstable();
    let mut distinct = 1;
    for w in nums.windows(2) {
        if w[1] != w[0] {
            distinct += 1;
        }
    }
    println!("{distinct}");
}
