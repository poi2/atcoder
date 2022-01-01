#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use num::integer::lcm;

fn main() {
    input! {
        a: u128,
        b: u128,
    }
    let ans = lcm(a, b);
    if ans > 10_u128.pow(18) {
        println!("Large");
    } else {
        println!("{}", ans);
    }
}
