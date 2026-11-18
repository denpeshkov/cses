//! https://cses.fi/problemset/task/1140

use std::{cmp::max, io::stdin};

fn main() {
    struct Project {
        start: usize,
        end: usize,
        reward: usize,
    }
    impl From<String> for Project {
        fn from(s: String) -> Self {
            let mut parts = s.split_whitespace();
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            let reward = parts.next().unwrap().parse().unwrap();
            Self { start, end, reward }
        }
    }

    let mut lines = stdin().lines();
    let n = lines.next().unwrap().unwrap().parse().unwrap();
    let mut projects: Vec<Project> = lines.take(n).map(|l| Project::from(l.unwrap())).collect();

    projects.sort_unstable_by_key(|p| p.end);

    let mut dp = vec![0; n];
    for i in 0..n {
        dp[i] = projects[i].reward;
        if i != 0 {
            dp[i] = max(dp[i], dp[i - 1]);
        }
        let j = projects[..i].partition_point(|p| p.end < projects[i].start);
        if j != 0 && projects[j - 1].end < projects[i].start {
            dp[i] = max(dp[i], dp[j - 1] + projects[i].reward)
        }
    }
    println!("{}", dp.iter().max().unwrap());
}
