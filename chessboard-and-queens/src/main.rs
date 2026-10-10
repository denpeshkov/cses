//! https://cses.fi/problemset/task/1624

use std::io::stdin;
fn main() {
    fn places(
        board: &Vec<String>,
        row: usize,
        cols: &mut [bool; 8],
        diag1: &mut [bool; 15],
        diag2: &mut [bool; 15],
    ) -> u64 {
        if row == 8 {
            return 1;
        }
        let mut count = 0;
        for col in 0..8 {
            if board[row].as_bytes()[col] == b'*'
                || cols[col]
                || diag1[row + col]
                || diag2[row + 7 - col]
            {
                continue;
            }
            cols[col] = true;
            diag1[row + col] = true;
            diag2[row + 7 - col] = true;
            count += places(board, row + 1, cols, diag1, diag2);
            cols[col] = false;
            diag1[row + col] = false;
            diag2[row + 7 - col] = false;
        }
        count
    }

    let board: Vec<String> = stdin().lines().map(|l| l.unwrap()).collect();

    let mut cols = [false; 8];
    let mut diag1 = [false; 15];
    let mut diag2 = [false; 15];

    let count = places(&board, 0, &mut cols, &mut diag1, &mut diag2);
    println!("{count}")
}
