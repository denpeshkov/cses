//! https://cses.fi/problemset/task/1640

use std::{
    cmp::Ordering::*,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let target_sum: usize = iter.next().unwrap().parse().unwrap();

    let mut nums: Vec<(usize, usize)> = iter
        .take(n)
        .enumerate()
        .map(|(i, p)| (i + 1, p.parse().unwrap()))
        .collect();

    nums.sort_unstable_by_key(|e| e.1);

    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        match (nums[l].1 + nums[r].1).cmp(&target_sum) {
            Less => l += 1,
            Equal => {
                println!("{} {}", nums[l].0, nums[r].0);
                return;
            }
            Greater => r -= 1,
        }
    }
    println!("IMPOSSIBLE");
}
