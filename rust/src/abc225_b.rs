// https://atcoder.jp/contests/abc222/tasks/abc222_d

use proconio::input;

fn main() {
    input! {
        n: usize,
        abn: [[usize; 2]; n-1],
    }
    let ab0 = &abn[0];

    let mut anses = [true, true];
    for ab in &abn {
        for i in 0..=1 {
            if !ab.contains(&ab0[i]) {
                anses[i] = false;
            }
        }
    }
    let ans = if anses.contains(&true) { "Yes" } else { "No" };
    println!("{}", ans);
}
