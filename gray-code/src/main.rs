//! https://cses.fi/problemset/task/2205

use std::io::stdin;

fn main() {
    let n: usize = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    for i in 0..2u32.pow(n as u32) {
        let code = i ^ (i >> 1);
        println!("{code:0n$b}")
    }
}
