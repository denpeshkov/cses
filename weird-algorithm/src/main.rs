//! https://cses.fi/problemset/task/1068

use std::{error, io};

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut n: i64 = io::stdin().lines().next().unwrap()?.parse()?;
    print!("{n}");
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
        print!(" {n}");
    }
    Ok(())
}
