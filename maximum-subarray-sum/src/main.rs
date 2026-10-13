//! https://cses.fi/problemset/task/1643

use std::{cmp::max, io::stdin};

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .take(n)
        .map(|e| e.parse().unwrap())
        .collect();

    let (mut sum, mut max_sum) = (0, i64::MIN);
    for n in nums {
        sum += n;
        sum = max(sum, n);
        max_sum = max(sum, max_sum);
    }
    println!("{max_sum}");
}
