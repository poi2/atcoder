// https://atcoder.jp/contests/abc132/tasks/abc132_c

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut d: [usize; n],
    }
    d.sort();
    let a = d[n / 2 - 1];
    let b = d[n / 2];
    println!("{}", b - a);
}
