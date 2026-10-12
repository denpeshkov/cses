//! https://cses.fi/problemset/task/1619

use std::{cmp::max, io::stdin};

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut arrivals: Vec<(usize, isize)> = lines
        .take(n)
        .flat_map(|l| {
            let l = l.unwrap();
            let (start, end) = l.split_once(' ').unwrap();
            [(start.parse().unwrap(), 1), (end.parse().unwrap(), -1)]
        })
        .collect();

    // If times are equal - +1 should come before -1 in sorted order.
    arrivals.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(b.1.cmp(&a.1)));

    let mut max_cnt: isize = 0;
    let mut cnt: isize = 0;
    for a in arrivals {
        cnt += a.1;
        max_cnt = max(max_cnt, cnt);
    }
    println!("{max_cnt}");
}
