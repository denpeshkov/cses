//! https://cses.fi/problemset/task/1193

use std::collections::VecDeque;
use std::io::{self, Read};

const DIRS: [(isize, isize, u8); 4] = [(-1, 0, b'U'), (0, 1, b'R'), (1, 0, b'D'), (0, -1, b'L')];

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();

    let n: usize = tokens.next().unwrap().parse().unwrap();
    let m: usize = tokens.next().unwrap().parse().unwrap();
    let mut grid: Vec<Vec<u8>> = (0..n)
        .map(|_| tokens.next().unwrap().as_bytes().to_vec())
        .collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for r in 0..n {
        for c in 0..m {
            match grid[r][c] {
                b'A' => start = (r, c),
                b'B' => end = (r, c),
                _ => continue,
            }
        }
    }

    let mut prev_step = vec![vec![0; m]; n];
    let mut queue = VecDeque::new();

    grid[start.0][start.1] = b'#';
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == end {
            break;
        }

        for (dir, &(dr, dc, _)) in DIRS.iter().enumerate() {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= n as isize || nc < 0 || nc >= m as isize {
                continue;
            }

            let (nr, nc) = (nr as usize, nc as usize);
            if grid[nr][nc] == b'#' {
                continue;
            }

            grid[nr][nc] = b'#';
            prev_step[nr][nc] = dir;
            queue.push_back((nr, nc));
        }
    }

    if grid[end.0][end.1] != b'#' {
        println!("NO");
        return;
    }

    let mut path = Vec::new();
    let mut cur = end;
    while cur != start {
        let (dr, dc, step) = DIRS[prev_step[cur.0][cur.1]];
        path.push(step);
        cur = (
            (cur.0 as isize - dr) as usize,
            (cur.1 as isize - dc) as usize,
        );
    }
    path.reverse();

    println!("YES");
    println!("{}", path.len());
    println!("{}", String::from_utf8(path).unwrap());
}
