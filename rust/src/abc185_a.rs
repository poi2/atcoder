// https://atcoder.jp/contests/abc185/tasks/abc185_a

use proconio::input;

fn main() {
    input! {
        a: [i32; 4]
    }
    let mut min: i32 = 100;
    for ai in a {
        if ai < min {
            min = ai;
        }
    }
    println!("{}", min);
}
