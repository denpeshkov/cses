//! https://cses.fi/problemset/task/1645

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
    let nums: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut mins = vec![0]; // we use 1 indexed in result
    for (i, &num) in nums.iter().enumerate() {
        loop {
            let mini = *mins.last().unwrap();
            if mini == 0 || num > nums[mini - 1] {
                break;
            }
            mins.pop();
        }
        print!("{} ", mins.last().unwrap());
        mins.push(i + 1);
    }
}
