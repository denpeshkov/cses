//! https://cses.fi/problemset/task/1085

use std::{
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, k) = (parse(), parse());
    let nums: Vec<usize> = iter::from_fn(|| Some(parse())).take(n).collect();

    let check = |m: usize| -> bool {
        let (mut sum, mut parts) = (0, 1);
        for num in &nums {
            sum += num;
            if sum > m {
                sum = *num;
                parts += 1;
            }
            if parts > k {
                return false;
            }
        }
        true
    };

    let (mut l, mut r): (usize, usize) = (*nums.iter().max().unwrap(), nums.iter().sum());
    while l < r {
        let m = l + (r - l) / 2;
        if check(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    println!("{l}");
}
