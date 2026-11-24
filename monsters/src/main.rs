//! https://cses.fi/problemset/task/1194

use std::{
    collections::VecDeque,
    io::{stdin, Read},
};

const DIRS: [(isize, isize, u8); 4] = [(-1, 0, b'U'), (1, 0, b'D'), (0, -1, b'L'), (0, 1, b'R')];

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    let mut tokens = input.split_whitespace();
    let mut read = || tokens.next().unwrap();

    let (n, m): (usize, usize) = (read().parse().unwrap(), read().parse().unwrap());
    let grid: Vec<Vec<_>> = (0..n).map(|_| read().as_bytes().to_vec()).collect();

    // BFS queue for monsters.
    let mut monsters_queue = VecDeque::new();
    // Stores min time when a monster reaches the cell.
    let mut monsters_time = vec![vec![usize::MAX; m]; n];
    // BFS queue for the hero.
    let mut queue = VecDeque::new();
    let mut start = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            if v == b'M' {
                // Add all monsters to BFS queue.
                monsters_queue.push_back((y, x, 0));
                monsters_time[y][x] = 0;
            }
            if v == b'A' {
                queue.push_back((y, x, 0));
                start = (y, x);
            }
        }
    }
    // Calculate monsters_time.
    while let Some((y, x, time)) = monsters_queue.pop_front() {
        for (dy, dx, _) in DIRS {
            let (y, x) = (y.wrapping_add_signed(dy), x.wrapping_add_signed(dx));
            if y >= n || x >= m {
                continue;
            }
            if grid[y][x] == b'#' || grid[y][x] == b'M' || monsters_time[y][x] <= time + 1 {
                continue;
            }
            monsters_time[y][x] = time + 1;
            monsters_queue.push_back((y, x, time + 1));
        }
    }

    let mut prev = vec![vec![usize::MAX; m]; n];
    let mut end = (usize::MAX, usize::MAX);
    // Calculate path for hero.
    while let Some((y, x, time)) = queue.pop_front() {
        if (grid[y][x] == b'.' || grid[y][x] == b'A')
            && (y == 0 || y == n - 1 || x == 0 || x == m - 1)
        {
            end = (y, x);
            break;
        }
        for (i, &(dy, dx, _)) in DIRS.iter().enumerate() {
            let (ny, nx) = (y.wrapping_add_signed(dy), x.wrapping_add_signed(dx));
            if ny >= n || nx >= m {
                continue;
            }
            // Skip cells where the monster arrives earlier than us.
            if grid[ny][nx] == b'#'
                || grid[ny][nx] == b'M'
                || monsters_time[ny][nx] <= time + 1
                || prev[ny][nx] != usize::MAX
            {
                continue;
            }
            prev[ny][nx] = i; // Index of direction.
            queue.push_back((ny, nx, time + 1));
        }
    }

    if end == (usize::MAX, usize::MAX) {
        println!("NO");
        return;
    }

    let mut path = vec![];
    while end != start {
        let (dy, dx, dir) = DIRS[prev[end.0][end.1]];
        path.push(dir);
        end = (
            (end.0 as isize - dy) as usize,
            (end.1 as isize - dx) as usize,
        );
    }
    path.reverse();
    println!("YES");
    println!("{}", path.len());
    for p in path {
        print!("{}", p as char);
    }
}
