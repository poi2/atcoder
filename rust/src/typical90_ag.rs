#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: f64,
        w: f64,
    }
    if n == 1.0 || w == 1.0 {
        println!("{}", (n * w) as usize);
    } else {
        let ans = ((w / 2.0).ceil() * (n / 2.0).ceil()) as usize;
        println!("{}", ans);
    }
}
