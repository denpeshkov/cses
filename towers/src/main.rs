//! https://cses.fi/problemset/task/1073

use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let n: usize = toks.next().unwrap().parse().unwrap();
    let cubes: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    let mut towers = vec![];
    for cube in cubes {
        let i = towers.partition_point(|&tower| tower <= cube);
        if i != towers.len() {
            towers[i] = cube
        } else {
            towers.push(cube);
        }
    }
    println!("{}", towers.len());
}
