//! https://cses.fi/problemset/task/2428/

use std::{
    collections::HashMap,
    io::{Read, stdin},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let n = parse();
    let k = parse();
    let nums: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    let mut count = 0;
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut i = 0;
    for (j, &num) in nums.iter().enumerate() {
        *map.entry(num).or_default() += 1;
        while map.len() > k {
            let cnt = map.get_mut(&nums[i]).unwrap();
            if *cnt == 1 {
                map.remove(&nums[i]);
            } else {
                *cnt -= 1;
            }
            i += 1;
        }
        if map.len() <= k {
            count += j - i + 1;
        }
    }
    println!("{count}");
}
