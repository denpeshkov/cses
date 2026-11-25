//! https://cses.fi/problemset/task/1679

use std::io::{stdin, Read};

use crate::State::*;

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
    for _ in 0..m {
        let (s, t) = parse();
        adj[s].push(t);
    }

    let mut state = vec![Unprocessed; n + 1];
    let mut ord = Vec::with_capacity(n + 1);
    for s in 1..=n {
        if matches!(state[s], Processed) {
            continue;
        }
        let topo_sortable = dfs(&adj, &mut state, &mut ord, s);
        if !topo_sortable {
            println!("IMPOSSIBLE");
            return;
        }
    }
    ord.reverse();
    for n in ord {
        print!("{n} ");
    }
}

#[derive(Clone, Copy, Debug)]
enum State {
    Unprocessed,
    Processing,
    Processed,
}

fn dfs(adj: &Vec<Vec<usize>>, state: &mut Vec<State>, ord: &mut Vec<usize>, s: usize) -> bool {
    state[s] = Processing;
    for &t in &adj[s] {
        if matches!(state[t], Processing) {
            return false; // Cycle detected!
        }
        if matches!(state[t], Processed) {
            continue;
        }
        if !dfs(adj, state, ord, t) {
            return false;
        }
    }
    state[s] = Processed;
    ord.push(s);
    true
}
