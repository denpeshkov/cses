//! https://cses.fi/problemset/task/1141

use std::{
    cmp,
    collections::HashMap,
    io::{Read, stdin},
};

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    let n: usize = toks.next().unwrap().parse().unwrap();
    let songs: Vec<usize> = toks.take(n).map(|t| t.parse().unwrap()).collect();

    let mut longest = 0usize;

    // Sliding window.
    let mut songs_wnd = HashMap::new();
    let mut start = 0usize;
    for (i, song) in songs.iter().enumerate() {
        if let Some(&j) = songs_wnd.get(&song) {
            while start <= j {
                songs_wnd.remove(&songs[start]);
                start += 1;
            }
        }
        songs_wnd.insert(song, i);
        longest = cmp::max(longest, i - start + 1);
    }
    println!("{longest}")
}
