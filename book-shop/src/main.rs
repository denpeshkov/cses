//! https://cses.fi/problemset/task/1158

use std::{
    cmp::max,
    io::{stdin, Read},
    iter,
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, total) = (parse(), parse());
    let prices: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();
    let pages: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    // dp[i] - max sum of pages that can be constructed from books with total sum of prices <=i.
    let mut dp = vec![0; total + 1];
    for i in 0..n {
        // We need to iterate in reverse order to count every price once.
        for j in (0..total).rev() {
            // Push DP.
            if j + prices[i] <= total {
                dp[j + prices[i]] = max(dp[j + prices[i]], dp[j] + pages[i]);
            }
        }
    }
    println!("{}", dp[total]);
}

// Another solution:
/*
fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, total) = (parse(), parse());
    let prices: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();
    let pages: Vec<_> = iter::from_fn(|| Some(parse())).take(n).collect();

    // dp[i][j] - max sum of pages for subset of books [0..i-1] with sum of prices <= j.
    //
    // We actually store only the last two rows here.
    let mut dp = vec![vec![0; total + 1]; 2];
    for i in 1..=n {
        let (curr, prev) = (i % 2, (i - 1) % 2); // Alternate between first and second row.
        for j in 0..=total {
            // Don't include the book.
            dp[curr][j] = dp[prev][j];
            if j >= prices[i - 1] {
                // Include the book.
                dp[curr][j] = max(dp[curr][j], dp[prev][j - prices[i - 1]] + pages[i - 1]);
            }
        }
    }
    println!("{}", dp[n % 2][total]);
}
*/
