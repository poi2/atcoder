#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

// NOTE DP で経路の総数を数え上げる。
// 通常の配る型の DP では TLE してしまうので、貰う型の DP に変形し、
// さらに通過できる経路を計算するやり方から、全体から通過できない経路を引く計算にする。
// 前提条件から、行けない経路の方が圧倒的に小さいため、この高速化が効果的。
fn main() {
    input! {
        n: usize, m: usize, k: usize,
        uvm: [(usize, usize); m],
    }

    const MOD: usize = 998244353;

    let mut dp = vec![0_usize; n];
    dp[0] = 1;

    let mut path = vec![vec![]; n];
    uvm.iter().for_each(|&(u, v)| {
        path[u - 1].push(v - 1);
        path[v - 1].push(u - 1);
    });
    (0..n).for_each(|i| path[i].push(i));

    (0..k).for_each(|_| {
        let total: usize = dp.iter().sum();
        dp = (0..n)
            .map(|i| path[i].iter().map(|&j| dp[j]).sum())
            .map(|sub: usize| (total - sub) % MOD )
            .collect();
    });

    println!("{}", dp[0]);
}
