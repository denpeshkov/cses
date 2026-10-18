//! https://cses.fi/problemset/task/2168

use std::{cmp, io::stdin};

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    #[derive(Copy, Clone)]
    struct Element {
        x: usize,
        y: usize,
        id: usize,
    }

    let mut ranges: Vec<Element> = lines
        .take(n)
        .enumerate()
        .map(|(i, line)| {
            let line = line.unwrap();
            let (s0, s1) = line.split_once(' ').unwrap();
            let (x, y): (usize, usize) = (s0.parse().unwrap(), s1.parse().unwrap());
            Element { x, y, id: i }
        })
        .collect();

    // Compare x in ASC order then tie-break y in DESC order.
    ranges.sort_unstable_by(|a, b| a.x.cmp(&b.x).then(b.y.cmp(&a.y)));

    let (mut contains, mut contained) = (vec![0u8; n], vec![0u8; n]);

    // Iterate from left to right.
    // If right endpoint <= maximum, the current interval is contained by another.
    let mut max_y = ranges[0].y;
    for r in &ranges[1..] {
        if r.y <= max_y {
            contained[r.id] = 1;
        }
        max_y = cmp::max(max_y, r.y)
    }

    // Iterate from right to left.
    // If the current right endpoint >= minimum, the current interval contains another.
    let mut min_y = ranges[n - 1].y;
    for r in ranges[..(n - 1)].iter().rev() {
        if r.y >= min_y {
            contains[r.id] = 1;
        }
        min_y = cmp::min(min_y, r.y)
    }

    for b in contains {
        print!("{b} ")
    }
    println!();
    for b in contained {
        print!("{b} ")
    }
}
