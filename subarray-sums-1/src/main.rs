//! https://cses.fi/problemset/task/1660

use std::{
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse();
    let target_sum = parse();
    let nums: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut count = 0;
    let (mut i, mut sum) = (0, 0);
    for (j, &num) in nums.iter().enumerate() {
        sum += num;
        while i <= j && sum > target_sum {
            sum -= nums[i];
            i += 1;
        }
        if sum == target_sum {
            count += 1;
        }
    }
    println!("{count}");
}
