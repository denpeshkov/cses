use std::io;

// https://cses.fi/problemset/task/1092
fn main() {
    let n: usize = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse()
        .unwrap();

    let (mut a, mut b) = (vec![], vec![]);
    let (mut sum_a, mut sum_b) = (0, 0);
    for num in (1..=n).rev() {
        if sum_a < sum_b {
            a.push(num);
            sum_a += num;
        } else {
            b.push(num);
            sum_b += num;
        }
    }
    if sum_a != sum_b {
        println!("NO");
        return;
    }
    println!("YES");
    println!("{}", a.len());
    for x in a {
        print!("{x} ");
    }
    println!();
    println!("{}", b.len());
    for x in b {
        print!("{x} ");
    }
    println!();
}
