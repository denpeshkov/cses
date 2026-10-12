//! https://cses.fi/problemset/task/1629

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut movies: Vec<(usize, usize)> = lines
        .take(n)
        .map(|l| {
            let l = l.unwrap();
            let (start, end) = l.split_once(' ').unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect();

    movies.sort_unstable_by_key(|m| m.1);

    let mut max_cnt = 0;
    let mut end_time = 0;
    for m in movies {
        if m.0 >= end_time {
            end_time = m.1;
            max_cnt += 1;
        }
    }
    println!("{max_cnt}")
}
