#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        k: u128,
    }
    let divs = divisors(k);
    let mut ans = 0;
    for i in 0..divs.len() {
        for j in i..divs.len() {
            let divs_k = k / (divs[i] * divs[j]);
            if divs[j] <= divs_k && k % (divs[i] * divs[j]) == 0 {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn divisors(k: u128) -> Vec<u128> {
    let mut i = 1;
    let mut ans = vec![];
    while i * i <= k {
        if k % i == 0 {
            ans.push(i);
            if i * i != k {
                ans.push(k / i);
            }
        }
        i += 1;
    }
    ans.sort();
    return ans;
}
