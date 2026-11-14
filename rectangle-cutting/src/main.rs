//! https://cses.fi/problemset/task/1744

use std::{
    cmp::min,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let (height, width) = input.split_once(' ').unwrap();
    let (height, width) = (
        height.trim().parse().unwrap(),
        width.trim().parse().unwrap(),
    );

    let mut dp = vec![vec![0usize; width + 1]; height + 1];
    for y in 1..=height {
        for x in 1..=width {
            if y == x {
                dp[y][x] = 0;
                continue;
            }
            dp[y][x] = usize::MAX;
            for i in 1..y {
                dp[y][x] = min(dp[y][x], 1 + dp[i][x] + dp[y - i][x])
            }
            for i in 1..x {
                dp[y][x] = min(dp[y][x], 1 + dp[y][i] + dp[y][x - i])
            }
        }
    }
    print!("{}", dp[height][width]);
}
