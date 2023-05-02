use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        _n: usize,
        s: String,
    }
    let usize_min = 0;
    let usize_max = (2 * 10_i32.pow(5) + 1) as usize;
    let (min, max) = s.split('-').fold((usize_max, usize_min), |(low, up), str| {
        let len = str.len();
        (min(low, len), max(up, len))
    });

    if min == max {
        println!("-1");
    } else {
        println!("{}", max);
    }
}
