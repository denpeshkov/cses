//! https://cses.fi/problemset/task/1623

use std::{
    cmp::min,
    io::{Read, stdin},
};
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let apples: Vec<u64> = input
        .split_whitespace()
        .skip(1) // skip n: the number of apples.
        .filter_map(|s| s.parse().ok())
        .collect();

    fn splits(apples: &[u64], sum1: u64, sum2: u64) -> u64 {
        if apples.is_empty() {
            return sum1.abs_diff(sum2);
        }
        let min1 = splits(&apples[1..], sum1 + apples[0], sum2);
        let min2 = splits(&apples[1..], sum1, sum2 + apples[0]);
        min(min1, min2)
    }
    println!("{}", splits(&apples, 0, 0))
}
