//! https://cses.fi/problemset/task/1630

use std::io::stdin;

fn main() {
    let mut lines = stdin().lines();
    let n = lines.next().unwrap().unwrap().parse().unwrap();
    let mut tasks: Vec<(usize, usize)> = lines
        .take(n)
        .map(|l| {
            let l = l.unwrap();
            let (du, dd) = l.split_once(' ').unwrap();
            (du.parse().unwrap(), dd.parse().unwrap())
        })
        .collect();

    // Assume t1<t2:
    // - First t1 then t2: (d1-t1) + (d2-(t1+t2)) = d1+d2-2t1-t2
    // - First t2 then t1: (d2-t2) + (d1-(t1+t2)) = d1+d2-t1-2t2
    // Thus, t1 must be first

    tasks.sort_unstable_by_key(|t| t.0);
    let (mut reward, mut cur) = (0, 0);
    for t in tasks {
        cur += t.0 as isize;
        reward += t.1 as isize - cur;
    }
    println!("{reward}");
}
