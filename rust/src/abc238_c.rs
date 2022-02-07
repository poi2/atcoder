#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: u128,
    }
    let mut max_digits = 0;
    let mut tmp_n = n;
    while tmp_n > 0 {
        max_digits += 1;
        tmp_n /= 10;
    }

    let ans = (1..=max_digits)
        .collect::<Vec<usize>>()
        .iter()
        .fold(0, |acc, &digit| {
            let current_max = (10_u128.pow(digit as u32) - 1).min(n);
            return solve(digit, current_max, acc);
        });
    println!("{}", ans);
}

fn dig_sum(digit: usize, max: u128) -> u128 {
    let d = digit as u32;
    let min = if d == 1 { 0 } else { 10_u128.pow(d - 1) - 1 };
    let limit = (10_u128.pow(d) - 1).min(max);
    let a1 = 1;
    let an = limit - min;

    return ((an + a1) * an) / 2;
}

fn solve(digit: usize, max: u128, acc: u128) -> u128 {
    let ret = dig_sum(digit, max);
    return (ret + acc) % 998244353;
}

// 1 -> 1
// 9 -> 9
// 10 -> 1
// 16 -> 7
// 99 -> 89
// 100 -> 1
// 999 -> 899
