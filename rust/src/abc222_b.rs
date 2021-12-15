// https://atcoder.jp/contests/abc222/tasks/abc222_b

// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        an: [usize; n],
    }
    let ans = an.iter().filter(|a| a < &&p).count();
    println!("{}", ans);
}
