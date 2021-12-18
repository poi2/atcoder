#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut an: [i64; n],
        mut bn: [i64; n],
    }
    an.sort();
    bn.sort();

    let ans: i64 = (0..n)
        .fold(0, |acc: i64, i: usize|
            acc + ((an[i] - bn[i])).abs()
        );
    println!("{}", ans);
}
