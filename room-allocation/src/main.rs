//! https://cses.fi/problemset/task/1164

use std::{cmp, io::stdin};

fn main() {
    let mut lines = stdin().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut events: Vec<_> = lines
        .take(n)
        .enumerate()
        .flat_map(|(i, l)| {
            let l = l.unwrap();
            let dd = l.split_once(' ').unwrap();
            let (arr, dep): (usize, usize) = (dd.0.parse().unwrap(), dd.1.parse().unwrap());
            [(arr, 1, i), (dep, -1, i)] // time, type, customer_idx
        })
        .collect();

    // Sort by time ASC, tie-break by arrival time.
    events.sort_unstable_by_key(|e| (e.0, cmp::Reverse(e.1)));

    let mut count = 0usize;
    let (mut free_rooms, mut choice) = (vec![], vec![0; n]);
    for e in events {
        if e.1 == 1 {
            let room = match free_rooms.pop() {
                Some(room) => room,
                None => {
                    count += 1;
                    count
                }
            };
            choice[e.2] = room;
        } else {
            free_rooms.push(choice[e.2]);
        }
    }
    println!("{count}");
    for r in choice {
        print!("{r} ");
    }
}
