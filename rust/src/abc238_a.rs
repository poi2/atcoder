#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
    }

    let ans = if n >= 2 && n <= 4 { "No" } else { "Yes" };
    println!("{}", ans);

    // 2^n > n^2
    // => n * log 2 > 2 * log n
    // => log 2 / 2 > log n / n

    // for i in 1..10 {
    //     let n = i as f32;
    //     println!("log {} / {} = {}", i, n, n.log2() / n);
    // }
    // println!("log 2 / 2 = {}", (2.0 as f32).log2() / 2.0);

    // log 1 / 1 = 0
    // log 2 / 2 = 0.5
    // log 3 / 3 = 0.52832085
    // log 4 / 4 = 0.5
    // log 5 / 5 = 0.4643856
    // log 6 / 6 = 0.4308271
    // log 7 / 7 = 0.40105072
    // log 8 / 8 = 0.375
    // log 9 / 9 = 0.3522139
    // log 2 / 2 = 0.5
}

// 2^n > n^2
// n * log 2 > 2 * log n
// log 2 / 2 > log n / n
// 2 > 1 = 3 > 4 > 5 > ... 1
