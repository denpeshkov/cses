//! https://cses.fi/problemset/task/2216

use std::io::{Read, stdin};

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut toks = buf.split_whitespace();
    let n: usize = toks.next().unwrap().parse().unwrap();
    let nums: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    let mut aux = vec![0; nums.len() + 1];
    for (i, &num) in nums.iter().enumerate() {
        aux[num] = i + 1;
    }

    let mut res = 1;
    for w in aux.windows(2) {
        if w[1] < w[0] {
            res += 1;
        }
    }
    println!("{res}")
}
