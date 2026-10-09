//! https://cses.fi/problemset/task/1755

use std::io::stdin;

fn main() {
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();
    let s = s.trim();

    let mut freq = [0; 26]; // map: character -> frequency
    for b in s.bytes() {
        freq[(b - b'A') as usize] += 1;
    }

    let mut palindrome = vec![b'*'; s.len()];
    let palindrome_len = palindrome.len();
    let mut idx = 0;
    for (i, f) in freq.iter().enumerate() {
        let (c, f) = (i as u8 + b'A', f);
        if f % 2 != 0 {
            if palindrome[palindrome_len / 2] != b'*' {
                println!("NO SOLUTION");
                return;
            }
            palindrome[palindrome_len / 2] = c;
        }
        for _ in 0..(*f / 2) {
            // if f is odd we floor.
            palindrome[idx] = c;
            palindrome[palindrome_len - 1 - idx] = c;
            idx += 1;
        }
    }
    println!("{}", String::from_utf8(palindrome).unwrap());
}
