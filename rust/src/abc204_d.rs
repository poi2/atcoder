#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut tn: [usize; n],
    }
    let sum = tn.iter().sum::<usize>();
    let max = sum + 1;

    let mut dp = vec![false; max];
    dp[0] = true;

    for i in 0..n {
        for j in (0..max).rev() {
            let t = tn[i];
            if j + t <= sum {
                dp[j + t] |= dp[j];
            }
        }
    }

    for i in (sum + 1) / 2..max {
        if dp[i] {
            println!("{}", i);
            return();
        }
    }
}
