#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u32,
        c: u64,
    };
    let ans = if a < c.pow(b) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
