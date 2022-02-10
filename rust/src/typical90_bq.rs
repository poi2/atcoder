#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::VecDeque;

const MOD: usize = 1000000007;

fn main() {
    input! {
        n: usize, k: usize,
    }

    let ans = match (n, k) {
        (1, k) => k,
        (_, 1) => 0,
        (2, k) => front(k),
        (n, k) => (front(k) * pow(k - 2, n - 2)) % MOD,
    };
    println!("{}", ans);
}

fn front(k: usize) -> usize {
    return (k * (k - 1)) % MOD;
}

// 繰り返し二乗法
fn pow(mut x: usize, mut e: usize) -> usize {
    let mut ans = 1;
    while e > 0 {
        if e % 2 == 1 {
            ans *= x;
            ans %= MOD;
        }
        x *= x;
        x %= MOD;
        e >>= 1;
    }
    return ans;
}
