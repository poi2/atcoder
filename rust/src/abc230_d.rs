#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    // NOTE 区間スケジューリング問題
    input! {
        n: usize, d: usize,
        mut lrn: [(usize, usize); n],
    }

    lrn.sort_by(|(_l1, r1), (_l2, r2)| r1.cmp(r2) );

    // let mut broken = 0;
    // let mut ans = 0;
    // for i in 0..n {
    //     if broken < lrn[i].0 {
    //         ans += 1;
    //         broken = lrn[i].1 + d - 1;
    //     }
    // }

    let (ans, _broken) = lrn
        .iter()
        .fold((0_usize, 0_usize), |(ans, broken), &(l, r)| {
            if broken < l {
                (ans + 1, r + d - 1)
            } else {
                (ans, broken)
            }
        });

    println!("{}", ans);
}
