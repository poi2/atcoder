#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        mut an: [usize; n],
    }
    let mut le = 0;
    let mut ri = l;

    while le + 1 < ri {
        let mid = (le + ri) / 2;
        let mut count = 0;
        let mut pre = 0;

        for i in 0..n {
            if an[i] - pre >= mid && l - an[i] >= mid {
                count += 1;
                pre = an[i];
            }
        }
        if count >= k {
            le = mid;
        } else {
            ri = mid;
        }

    }
    println!("{}", le);
}
