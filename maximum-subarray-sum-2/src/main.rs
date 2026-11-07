//! https://cses.fi/problemset/task/1644

use std::{
    cmp::max,
    collections::VecDeque,
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let mut parse = || toks.next().unwrap().parse::<isize>().unwrap();

    let (n, min_len, max_len) = (parse() as usize, parse() as usize, parse() as usize);
    let nums: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut ps = vec![0; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        ps[i + 1] = ps[i] + num
    }

    let mut order: VecDeque<_> = VecDeque::new();

    let mut res = isize::MIN;
    for i in 1..=nums.len() {
        if i >= min_len {
            while order.back().is_some_and(|&v| v > ps[i - min_len]) {
                order.pop_back();
            }
            order.push_back(ps[i - min_len]);
            res = max(res, ps[i] - order.front().unwrap());
        }
        if i >= max_len && *order.front().unwrap() == ps[i - max_len] {
            order.pop_front();
        }
    }
    println!("{res}");
}
