//! https://cses.fi/problemset/task/1070

use std::{error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut n = String::new();
    io::stdin().read_line(&mut n)?;
    let n: u64 = n.trim().parse()?;

    if n == 1 {
        print!("1");
        return Ok(());
    } else if n <= 3 {
        print!("NO SOLUTION");
        return Ok(());
    }

    let mut num = 2;
    for _ in 0..n {
        print!("{num} ");
        num += 2;
        if num > n {
            num = 1
        }
    }
    Ok(())
}
