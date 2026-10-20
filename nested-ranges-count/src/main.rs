//! https://cses.fi/problemset/task/2169

use std::{cmp, io::stdin, ops::RangeTo};

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut ranges: Vec<_> = lines
        .take(n)
        .enumerate()
        .map(|(i, l)| {
            let l = l.unwrap();
            let (s1, s2) = l.split_once(' ').unwrap();
            let (x, y) = (s1.parse().unwrap(), s2.parse().unwrap());
            Range { x, y, idx: i }
        })
        .collect();

    // Compress coordinates.
    let max_coord = compress(&mut ranges);
    // Compare by x in ASC order, tie-break by y in DESC order.
    ranges.sort_unstable_by_key(|r| (r.x, cmp::Reverse(r.y)));

    let mut bit = Bit::new(max_coord);
    let mut contains = vec![0; n];
    for range in ranges.iter().rev() {
        let num = bit.sum(..range.y + 1);
        contains[range.idx] = num;
        bit.add(range.y, 1);
    }
    for v in contains {
        print!("{v} ");
    }
    println!();

    let mut bit = Bit::new(max_coord);
    let mut contained = vec![0; n];
    for range in ranges {
        // Total processed so far minus the ones strictly less than y.
        let total = bit.sum(..max_coord);
        let less = bit.sum(..range.y);
        contained[range.idx] = total - less;
        bit.add(range.y, 1);
    }
    for v in contained {
        print!("{v} ");
    }
}

struct Range {
    x: usize,
    y: usize,
    idx: usize,
}

// Fenwick-Tree
struct Bit(Vec<usize>);

impl Bit {
    fn new(n: usize) -> Self {
        Self(vec![0; n + 1])
    }

    fn add(&mut self, idx: usize, x: usize) {
        let mut idx = idx + 1;
        while idx < self.0.len() {
            self.0[idx] += x;
            idx += idx & idx.wrapping_neg();
        }
    }

    fn sum(&self, range: RangeTo<usize>) -> usize {
        let mut idx = range.end;
        let mut sum = 0;
        while idx > 0 {
            sum += self.0[idx];
            idx -= idx & idx.wrapping_neg();
        }
        sum
    }
}

fn compress(ranges: &mut [Range]) -> usize {
    let mut coords: Vec<_> = ranges.iter().flat_map(|r| [r.x, r.y]).collect();
    coords.sort_unstable();
    coords.dedup();
    for r in ranges.iter_mut() {
        r.x = coords.binary_search(&r.x).unwrap();
        r.y = coords.binary_search(&r.y).unwrap();
    }
    coords.len()
}
