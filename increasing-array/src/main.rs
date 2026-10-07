//! https://cses.fi/problemset/task/1094

use std::{
    error::Error,
    io::{self, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let nums: Vec<u64> = input
        .split_whitespace()
        .skip(1) // skip n: the size of the array
        .filter_map(|s| s.parse().ok())
        .collect();
    let mut moves = 0;
    let mut prev = nums[0];
    for num in nums[1..].iter().cloned() {
        if num < prev {
            moves += prev - num;
        } else {
            prev = num;
        }
    }
    println!("{moves}");
    Ok(())
}
