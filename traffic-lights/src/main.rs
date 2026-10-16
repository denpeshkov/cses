//! https://cses.fi/problemset/task/1163

use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, stdin},
    ops::Bound::{Excluded, Unbounded},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let length: usize = toks.next().unwrap().parse().unwrap();
    let n: usize = toks.next().unwrap().parse().unwrap();
    let positions: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    let mut positions_set: BTreeSet<usize> = BTreeSet::new();
    let mut lengths_multiset: BTreeMap<usize, usize> = BTreeMap::new();
    positions_set.insert(0);
    positions_set.insert(length);
    lengths_multiset.insert(length, 1);

    for pos in positions {
        let left = positions_set.range(..pos).next_back().unwrap();
        let right = positions_set
            .range((Excluded(pos), Unbounded))
            .next()
            .unwrap();

        let length = right - left;
        if let Some(e) = lengths_multiset.get_mut(&length) {
            *e -= 1;
            if *e == 0 {
                lengths_multiset.remove(&length);
            }
        }
        *lengths_multiset.entry(pos - left).or_default() += 1;
        *lengths_multiset.entry(right - pos).or_default() += 1;

        positions_set.insert(pos);

        println!("{}", lengths_multiset.last_key_value().unwrap().0)
    }
}
