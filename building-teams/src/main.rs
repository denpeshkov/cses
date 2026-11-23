//! https://cses.fi/problemset/task/1668

use std::io::{stdin, Read};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || {
        (
            toks.next().unwrap().parse().unwrap(),
            toks.next().unwrap().parse().unwrap(),
        )
    };

    let (n, m): (usize, usize) = parse();
    let mut adj: Vec<Vec<_>> = vec![vec![]; n + 1];
    for _ in 0..m {
        let (s, t) = parse();
        adj[s].push(t);
        adj[t].push(s);
    }

    let mut color = vec![0; n + 1];
    for s in 1..=n {
        if color[s] == 0 {
            color[s] = 1;
            if !dfs(&adj, &mut color, s) {
                println!("IMPOSSIBLE");
                return;
            };
        }
    }
    for c in &color[1..] {
        print!("{c} ");
    }
}

fn dfs(adj: &Vec<Vec<usize>>, color: &mut Vec<u8>, s: usize) -> bool {
    for &t in &adj[s] {
        if color[t] == color[s] {
            // Impossible to bi-partite.
            return false;
        }
        if color[t] == 0 {
            // Unvisited.
            color[t] = color[s] ^ 3; // 1->2 and 2->1
            if !dfs(adj, color, t) {
                return false;
            };
        }
    }
    true
}
