#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    let div = 10_usize.pow(9) + 7;
    input! {
        k: usize,
    }

    if k % 9 != 0 {
        println!("0");
    } else {
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 0..=k {
            for j in 1..=9.min(i) {
                dp[i] = (dp[i] + dp[i - j]) % div;
            }
        }
        println!("{}", dp[k]);
    }
}
