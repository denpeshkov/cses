//! https://cses.fi/problemset/task/1632

use std::{
    collections::BTreeSet,
    io::{stdin, Read},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let mut parse = || toks.next().unwrap().parse().unwrap();

    let (n, k): (usize, usize) = (parse(), parse());
    let mut movies: Vec<(usize, usize)> = (0..n).map(|_| (parse(), parse())).collect();

    movies.sort_unstable_by_key(|&(_, end)| end);

    // (time the member is free, member id)
    let mut free_at: BTreeSet<(usize, usize)> = (0..k).map(|id| (0, id)).collect();

    let mut res = 0;
    for (start, end) in movies {
        // Take the member who got free the latest, but no later than the movie start.
        let Some(&(time, id)) = free_at.range(..=(start, usize::MAX)).next_back() else {
            continue;
        };
        free_at.remove(&(time, id));
        free_at.insert((end, id));
        res += 1;
    }
    println!("{res}");
}
