//! https://cses.fi/problemset/task/1069

use std::{cmp::max, error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    let mut str = String::new();
    io::stdin().read_line(&mut str)?;
    let mut chars = str.char_indices();

    let mut longest = 0;
    let (mut prev_i, mut prev_c) = chars.next().ok_or("empty")?;
    for (i, c) in chars {
        if c != prev_c {
            longest = max(longest, i - prev_i);
            prev_i = i;
            prev_c = c;
        }
    }
    println!("{longest}");
    Ok(())
}
