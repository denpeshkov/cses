//! https://cses.fi/problemset/task/1639

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let s = lines.next().unwrap().unwrap();
    let t = lines.next().unwrap().unwrap();
    let s = s.as_bytes();
    let t = t.as_bytes();

    // dp[i]j[] - min edits to convert from s[0..i-1] to t[0..j-1].
    // if i=0 or j=0 - means to convert from/to emtpy string.
    let mut dp = vec![vec![0usize; t.len() + 1]; s.len() + 1];
    for (i, v) in dp[0].iter_mut().enumerate() {
        *v = i;
    }
    for (i, v) in dp.iter_mut().enumerate() {
        v[0] = i;
    }
    for i in 1..=s.len() {
        for j in 1..=t.len() {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]);
            }
        }
    }
    println!("{}", dp[s.len()][t.len()]);
}
