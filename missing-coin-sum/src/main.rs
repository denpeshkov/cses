//! https://cses.fi/problemset/task/2183

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut coins: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .take(n)
        .map(|e| e.parse().unwrap())
        .collect();

    coins.sort_unstable();

    let mut min_sum = 0;
    for c in coins {
        if c > min_sum + 1 {
            break;
        }
        min_sum += c;
    }
    println!("{}", min_sum + 1);
}
