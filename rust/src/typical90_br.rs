#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(isize, isize); n],
    }
    let mut xn = vec![];
    let mut yn = vec![];
    for (x, y) in xyn {
        xn.push(x);
        yn.push(y);
    }

    println!("{}", manhattan_dist(xn) + manhattan_dist(yn));
}

fn manhattan_dist(mut list: Vec<isize>) -> usize {
    list.sort();
    let mid = list[list.len() / 2];
    let mut ans = 0;
    for i in 0..list.len() {
        if list[i] < mid {
            ans += (mid - list[i]).abs();
        } else if list[i] > mid {
            ans += (list[i] - mid).abs();
        }
    }
    return ans as usize;
}
