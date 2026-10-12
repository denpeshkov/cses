//! https://cses.fi/problemset/task/1091

use std::{
    collections::{BTreeMap, btree_map::Entry},
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    let tickets: usize = toks.next().unwrap().parse().unwrap();
    let customers: usize = toks.next().unwrap().parse().unwrap();
    let prices: Vec<usize> = toks
        .by_ref()
        .take(tickets)
        .map(|s| s.parse().unwrap())
        .collect();
    let max_prices: Vec<usize> = toks
        .by_ref()
        .take(customers)
        .map(|s| s.parse().unwrap())
        .collect();

    let mut prices = prices.into_iter().fold(BTreeMap::new(), |mut map, item| {
        *map.entry(item).or_insert(0) += 1usize;
        map
    });

    for mp in &max_prices {
        match prices.range(..=mp).next_back().map(|(&k, _)| k) {
            Some(key) => {
                if let Entry::Occupied(mut e) = prices.entry(key) {
                    println!("{}", key);
                    *e.get_mut() -= 1;
                    if *e.get() == 0 {
                        e.remove();
                    }
                }
            }
            None => println!("-1"),
        }
    }
}
