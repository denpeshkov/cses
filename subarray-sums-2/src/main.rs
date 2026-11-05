//! https://cses.fi/problemset/task/1661

use std::{
    collections::HashMap,
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse::<i64>().unwrap();

    let n = parse();
    let target_sum = parse();
    let nums: Vec<_> = iter::from_fn(|| Some(parse())).take(n as usize).collect();

    let mut count = 0;
    let mut map = HashMap::with_capacity(nums.len());
    map.insert(0, 1i64);
    let mut sum = 0;
    for num in nums {
        sum += num;
        let pref_sum = sum - target_sum;
        count += map.get(&pref_sum).unwrap_or(&0);
        *map.entry(sum).or_default() += 1;
    }
    println!("{count}");
}
