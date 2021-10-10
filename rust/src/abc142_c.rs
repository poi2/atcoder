// https://atcoder.jp/contests/abc142/tasks/abc142_c

use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut list: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        list.push((a[i], i));
    }
    println!("{:?}", list);
}