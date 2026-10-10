//! https://cses.fi/problemset/task/1622

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut perm: Vec<u8> = s.trim().into();

    perm.sort_unstable();

    let mut perms = vec![String::from_utf8(perm.clone()).unwrap()];
    while let Some(next) = next_perm(&mut perm) {
        perms.push(next);
    }

    println!("{}", perms.len());
    for p in perms {
        println!("{p}");
    }
}

fn next_perm(perm: &mut [u8]) -> Option<String> {
    // Index i of the rightmost element such that perm[i] < perm[i+1]
    let i = perm.windows(2).rposition(|w| w[0] < w[1])?;
    // Index j of the rightmost element such that perm[j] > perm[i]
    let j = perm.iter().rposition(|e| e > &perm[i]).unwrap();

    perm.swap(i, j);
    perm[i + 1..].reverse();
    Some(String::from_utf8_lossy(perm).to_string())
}
