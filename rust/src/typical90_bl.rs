#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize, q: usize,
        an: [isize; n],
        qs: [(usize, usize, isize); q]
    }
    let mut gaps: Vec<isize> = an.iter().tuple_windows().map(|(s, t)| t - s ).collect();
    let mut ans: isize = gaps.iter().map(|i| i.abs()).sum();
    // println!("{:?} {}", gaps, ans);

    for (rawl, rawr, v) in qs {
        if rawl > 1 {
            let l = rawl - 2;
            ans -= gaps[l].abs();
            gaps[l] += v;
            ans += gaps[l].abs();
        }
        if rawr < n {
            let r = rawr - 1;
            ans -= gaps[r].abs();
            gaps[r] -= v;
            ans += gaps[r].abs();
        }
        // println!("{:?} {}", gaps, ans);

        println!("{}", ans);
    }
}
