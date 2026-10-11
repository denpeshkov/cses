//! https://cses.fi/problemset/task/1084

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let num_applicants: usize = toks.next().unwrap().parse().unwrap();
    let num_appartments: usize = toks.next().unwrap().parse().unwrap();
    let max_diff: usize = toks.next().unwrap().parse().unwrap();
    let mut desired_sizes: Vec<usize> = toks
        .by_ref()
        .take(num_applicants)
        .map(|s| s.parse().unwrap())
        .collect();
    let mut sizes: Vec<usize> = toks
        .take(num_appartments)
        .map(|s| s.parse().unwrap())
        .collect();

    desired_sizes.sort_unstable();
    sizes.sort_unstable();

    let mut cnt = 0;
    let (mut i, mut j) = (0, 0);
    while i < sizes.len() && j < desired_sizes.len() {
        if sizes[i] + max_diff < desired_sizes[j] {
            i += 1
        } else if sizes[i] > desired_sizes[j] + max_diff {
            j += 1;
        } else {
            i += 1;
            j += 1;
            cnt += 1;
        }
    }
    println!("{cnt}");
}
