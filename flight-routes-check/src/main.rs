//! https://cses.fi/problemset/task/1682

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
    let (mut adj, mut adj_t) = (vec![vec![]; n + 1], vec![vec![]; n + 1]);
    for _ in 0..m {
        let (s, t) = parse();
        adj[s].push(t);
        adj_t[t].push(s);
    }

    let mut visited = vec![false; n + 1];
    let mut verts = Vec::with_capacity(n);
    for s in 1..=n {
        if visited[s] {
            continue;
        }
        // First traverse the transposed graph, so that first found SCC - is a sink component.
        dfs(&adj_t, &mut visited, &mut verts, s);
    }

    visited.fill(false);
    let mut scc = Vec::with_capacity(n);
    for &s in verts.iter().rev() {
        if visited[s] {
            continue;
        }
        if !scc.is_empty() && s != scc[0] {
            println!("NO");
            println!("{} {}", scc[0], s);
            return;
        }
        dfs(&adj, &mut visited, &mut scc, s);
    }
    println!("YES");
}

fn dfs(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, verts: &mut Vec<usize>, s: usize) {
    visited[s] = true;
    for &t in &adj[s] {
        if visited[t] {
            continue;
        }
        dfs(adj, visited, verts, t)
    }
    verts.push(s);
}
