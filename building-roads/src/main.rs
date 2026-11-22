//! https://cses.fi/problemset/task/1666

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let mut parse = || {
        let s = lines.next().unwrap().unwrap();
        let (a, b) = s.split_once(' ').unwrap();
        (a.parse().unwrap(), b.parse().unwrap())
    };

    let (n, m): (usize, usize) = parse();
    let mut adj: Vec<Vec<_>> = vec![vec![]; n];
    for _ in 0..m {
        let (s, t) = parse();
        let (s, t) = (s - 1, t - 1);
        adj[s].push(t);
        adj[t].push(s);
    }

    let mut roads = Vec::new();
    let mut cons = vec![0; n];
    for i in 0..n {
        if cons[i] == 0 {
            if i > 0 {
                roads.push((cons[i - 1], i + 1));
            }
            dfs(&adj, &mut cons, i, i + 1);
        }
    }
    println!("{}", roads.len());
    for r in roads {
        println!("{} {}", r.0, r.1);
    }
}

fn dfs(adj: &Vec<Vec<usize>>, cons: &mut Vec<usize>, s: usize, component: usize) {
    cons[s] = component;
    for &t in &adj[s] {
        if cons[t] == 0 {
            dfs(adj, cons, t, component);
        }
    }
}
