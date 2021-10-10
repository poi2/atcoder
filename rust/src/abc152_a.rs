// https://atcoder.jp/contests/abc152/tasks/abc152_a
use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }
    if n == m {
        println!("Yes");
    } else {
        println!("No");
    }
}
