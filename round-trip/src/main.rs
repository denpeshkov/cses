//! https://cses.fi/problemset/task/1669

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

    let mut parents = vec![0; n + 1];
    for s in 1..=n {
        if parents[s] != 0 {
            continue;
        }
        parents[s] = s; // Mark node as visited.
        let mut end = dfs(&adj, &mut parents, s);
        if end != 0 {
            let start = end; // start and end are the sam in the cycle.
            let mut path = vec![];
            loop {
                path.push(end);
                end = parents[end];
                if end == start {
                    break;
                }
            }
            path.push(start);
            path.reverse();
            println!("{}", path.len());
            for v in path {
                print!("{v} ");
            }
            return;
        }
    }
    println!("IMPOSSIBLE");
}

fn dfs(adj: &Vec<Vec<usize>>, parents: &mut Vec<usize>, s: usize) -> usize {
    for &t in &adj[s] {
        if parents[t] == 0 {
            parents[t] = s;
            let n = dfs(adj, parents, t);
            if n != 0 {
                return n;
            }
        } else if t != parents[s] {
            // Found cycle.
            parents[t] = s;
            return t;
        }
    }
    0
}
