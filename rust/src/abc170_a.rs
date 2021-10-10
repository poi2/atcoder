// https://atcoder.jp/contests/abc170/tasks/abc170_a
use proconio::input;

fn main() {
    input! {
        x: [i32; 5]
    }
    for i in 0..5 {
        if x[i] == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
