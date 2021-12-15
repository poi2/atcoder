// https://atcoder.jp/contests/abc222/tasks/abc222_c

// use itertools::Itertools;
use proconio::input;
use proconio::marker::{Chars};

fn vs(a: char, b: char) -> i32 {
    if a == b {
        return 0
    }
    if (a == 'G' && b == 'C') || (a == 'C' && b == 'P') || (a == 'P' && b == 'G') {
        return 1
    }
    return -1
}

fn main() {
    let max: usize = 100;
    input! {
        n: usize,
        m: usize,
    }
    let mut a = Vec::new();
    for i in 0..2 * n {
        input! {
            s: Chars,
        }
        let tap: (usize, usize, Vec<char>) = (max, i + 1, s);
        a.push(tap);
    }

    for i in 0..m {
        for k in 0..n {
            let resp = vs(a[k*2].2[i], a[k*2+1].2[i]);
            if resp == 1 {
                a[k*2].0 -= 1;
            } else if resp == -1 {
                a[k*2+1].0 -= 1;
            }
        }
        a.sort();
    }

    for i in 0..2 * n {
        println!("{}", a[i].1);
    }
}
