#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize, x: usize,
        abn: [(usize, usize); n],
    }

    let mut dp0 = HashSet::new();
    let mut dp1 = HashSet::new();
    dp0.insert(0);
    for (a, b) in abn {
        for i in dp0.iter() {
            dp1.insert(a + i);
            dp1.insert(b + i);
        }
        dp0 = dp1;
        dp1 = HashSet::new();
    }

    match dp0.get(&x) {
        Some(_) => println!("Yes"),
        None => println!("No"),
    }
}
