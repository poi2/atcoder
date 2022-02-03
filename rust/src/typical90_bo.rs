#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    input! {
        n: u128, k: usize,
    }
    let ans = (0..k).fold(n, |acc, _| base_10_to_9(base_8_to_10(acc)));
    println!("{}", ans);
}

fn base_8_to_10(mut o: u128) -> u128 {
    let mut i = 0;
    let mut ans = 0;
    while o > 0 {
        let rem = o % 10;
        ans += (8_usize.pow(i) as u128) * rem;
        i += 1;
        o /= 10;
    }
    return ans;
}

fn base_10_to_9(mut d: u128) -> u128 {
    let mut ans = 0;
    let mut i = 0;
    while d > 0 {
        let rem = d % 9;
        let rem85 = if rem == 8 { 5 } else { rem };
        ans += (10_usize.pow(i) as u128) * rem85;
        i += 1;
        d /= 9;
    }
    return ans;
}
