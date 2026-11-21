//! https://cses.fi/problemset/task/1192

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, m) = (parse(), parse());
    let mut map: Vec<_> = (0..n)
        .map(|_| toks.next().unwrap().to_string().into_bytes())
        .collect();

    fn dfs(map: &mut Vec<Vec<u8>>, r: usize, c: usize) {
        if map[r][c] == b'#' {
            return;
        }
        map[r][c] = b'#'; // Mark as visited
        if r >= 1 {
            dfs(map, r - 1, c); // Up
        }
        if r < map.len() - 1 {
            dfs(map, r + 1, c); // Down
        }
        if c >= 1 {
            dfs(map, r, c - 1); // Left
        }
        if c < map[r].len() - 1 {
            dfs(map, r, c + 1); // Right
        }
    }

    let mut cnt = 0;
    for r in 0..n {
        for c in 0..m {
            if map[r][c] == b'#' {
                continue;
            }
            cnt += 1;
            dfs(&mut map, r, c);
        }
    }
    println!("{cnt}");
}
