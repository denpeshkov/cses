//! https://cses.fi/problemset/task/1683

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
    let mut adj = vec![vec![]; n + 1];
    let mut adj_t = vec![vec![]; n + 1];
    for _ in 0..m {
        let (s, t) = parse();
        adj[s].push(t);
        adj_t[t].push(s);
    }

    let mut visited = vec![false; n + 1];
    let mut verts = vec![];
    for s in 1..=n {
        if visited[s] {
            continue;
        }
        dfs(&adj_t, &mut visited, s, &mut verts);
    }

    let mut scc = vec![0; n + 1];
    let mut c = 0;
    for &s in verts.iter().rev() {
        if scc[s] != 0 {
            continue;
        }
        c += 1;
        scc[s] = c;
        dfs2(&adj, &mut scc, s);
    }
    println!("{}", c);
    for n in &scc[1..] {
        print!("{n} ");
    }
}

fn dfs(adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>, s: usize, out: &mut Vec<usize>) {
    if visited[s] {
        return;
    }
    visited[s] = true;
    for &t in &adj[s] {
        dfs(adj, visited, t, out);
    }
    out.push(s);
}

fn dfs2(adj: &Vec<Vec<usize>>, scc: &mut Vec<usize>, s: usize) {
    for &t in &adj[s] {
        if scc[t] != 0 {
            continue;
        }
        scc[t] = scc[s];
        dfs2(adj, scc, t);
    }
}
