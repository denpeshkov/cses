//! https://cses.fi/problemset/task/1083

use std::{
    error::Error,
    io::{self},
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut lines = io::stdin().lines();

    let n: usize = lines.next().unwrap()?.parse()?;

    let want_sum = n * (n + 1) / 2;
    let mut sum = 0;
    for num in lines.next().unwrap()?.split_whitespace().take(n - 1) {
        let num: usize = num.parse()?;
        sum += num;
    }
    println!("{}", want_sum - sum);

    Ok(())
}
