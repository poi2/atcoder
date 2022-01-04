#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }
    let mut dp = [0_u128; 7];
    let chars = s
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let div = 10_u128.pow(9) + 7;

    for i in 0..n {
        let c: &str = &chars[i];
        match c {
            "a" => dp[0] += 1,
            "t" => dp[1] = (dp[1] + dp[0]) % div,
            "c" => dp[2] = (dp[2] + dp[1]) % div,
            "o" => dp[3] = (dp[3] + dp[2]) % div,
            "d" => dp[4] = (dp[4] + dp[3]) % div,
            "e" => dp[5] = (dp[5] + dp[4]) % div,
            "r" => dp[6] = (dp[6] + dp[5]) % div,
            _ => (),
        }
    }
    println!("{}", dp[6]);
}
