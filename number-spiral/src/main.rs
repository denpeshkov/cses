//! https://cses.fi/problemset/task/1071

use std::{cmp, error::Error, io};

fn main() -> Result<(), Box<dyn Error>> {
    for line in io::stdin().lines().skip(1) {
        let line = line.unwrap();
        let (row, col) = line.split_once(' ').unwrap();
        let (mut row, mut col) = (row.parse()?, col.parse()?);

        let layer: u64 = cmp::max(row, col);

        if layer % 2 != 0 {
            std::mem::swap(&mut row, &mut col);
        }
        let num = if col == layer {
            (layer - 1).pow(2) + 1 + (row - 1)
        } else {
            layer.pow(2) - (col - 1)
        };
        println!("{num}");
    }
    Ok(())
}
