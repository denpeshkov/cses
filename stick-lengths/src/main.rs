//! https://cses.fi/problemset/task/1074

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut sticks: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .take(n)
        .map(|e| e.parse().unwrap())
        .collect();

    sticks.sort_unstable();
    let len = sticks.len() / 2;
    let (_, median, _) = sticks.select_nth_unstable(len);
    let median = *median; // Make borrowck happy.
    let min_cost = sticks
        .into_iter()
        .fold(0, |acc, v| acc + median.abs_diff(v));
    println!("{min_cost}");
}
