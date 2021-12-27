// https://atcoder.jp/contests/typical90/tasks/typical90_x

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: i32,
        an: [i32; n],
        bn: [i32; n],
    }
    for i in 0..n {
        k -= (an[i] - bn[i]).abs();
    }
    let ans = if k % 2 == 0 && k >= 0 { "Yes" } else { "No" };
    println!("{}", ans);
}
