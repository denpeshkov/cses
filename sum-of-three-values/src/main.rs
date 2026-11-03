//! https://cses.fi/problemset/task/1641

use std::{
    cmp::Ordering::{Equal, Greater, Less},
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let n = toks.next().unwrap().parse().unwrap();
    let target_sum: usize = toks.next().unwrap().parse().unwrap();
    let mut nums: Vec<(usize, usize)> = toks
        .take(n)
        .enumerate()
        .map(|(i, v)| (v.parse().unwrap(), i + 1))
        .collect();

    nums.sort_unstable();

    for i in 0..nums.len().saturating_sub(2) {
        let (mut j, mut k) = (i + 1, nums.len().saturating_sub(1));
        while j < k {
            match target_sum.cmp(&(nums[i].0 + nums[j].0 + nums[k].0)) {
                Less => k -= 1,
                Equal => {
                    println!("{} {} {}", nums[i].1, nums[j].1, nums[k].1,);
                    return;
                }
                Greater => j += 1,
            }
        }
    }
    println!("IMPOSSIBLE");
}
