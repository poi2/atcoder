#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
    }
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    let div = 10_usize.pow(9) + 7;
    for i in 1..=n {
        if i < l {
            dp[i] = dp[i - 1] % div;
        } else {
            dp[i] = (dp[i - 1] + dp[i - l]) % div;
        }
    }
    println!("{}", dp[n]);
}
