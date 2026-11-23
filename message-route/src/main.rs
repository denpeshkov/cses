//! https://cses.fi/problemset/task/1667

use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

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

    let mut queue = VecDeque::new();
    let mut parents = vec![0; n + 1];
    queue.push_back(1); // Add starting node.
    parents[1] = 1; // Starting node has no parent.

    while let Some(s) = queue.pop_front() {
        for &t in &adj[s] {
            if parents[t] == 0 {
                parents[t] = s;
                queue.push_back(t);
            }
        }
    }

    if parents[n] == 0 {
        println!("IMPOSSIBLE");
        return;
    }
    let mut path = Vec::new();
    let mut n = n;
    while parents[n] != n {
        path.push(n);
        n = parents[n];
    }
    path.push(1); // Add starting node.
    path.reverse();
    println!("{}", path.len());
    for n in path {
        print!("{n} ");
    }
}
