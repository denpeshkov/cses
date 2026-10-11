//! https://cses.fi/problemset/task/1090

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let n: usize = toks.next().unwrap().parse().unwrap();
    let max_weight: usize = toks.next().unwrap().parse().unwrap();
    let mut weights: Vec<usize> = toks.by_ref().take(n).map(|s| s.parse().unwrap()).collect();

    weights.sort_unstable();

    let mut cnt = 0;
    let (mut l, mut r) = (0, weights.len() - 1);
    while l <= r {
        if l == r {
            // usize can't be negative
            cnt += 1;
            break;
        }
        if weights[l] + weights[r] > max_weight {
            r -= 1;
        } else {
            l += 1;
            r -= 1;
        }
        cnt += 1
    }
    println!("{cnt}");
}
