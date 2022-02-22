#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize, d: usize,
    }
    const MOD: u128 = 998244353;
    let mut pows = vec![0_u128; n.max(d) + 1];
    pows[0] = 1;
    for i in 1..pows.len() {
        pows[i] = (pows[i - 1] * 2) % MOD;
    }

    let mut ans = 0;

    // NOTE 部分木の頂点に一方を、もう一方を部分木の頂点から d の距離取るケース
    // 距離が木の最大の深さよりも小さいときに再現可能
    if n > d {
        let top = pows[n - d] - 1;
        let left = pows[d];
        let right = 1;
        ans += top * left * right;
        ans %= MOD;
    }

    // NOTE 部分木の左右に分岐するケース
    // 左側の長さを 1..d（右側は逆算で求まる）まで変化させ、そのとき左サイドと右サイドの組み合わせの数を求める
    // 左右の腕の長い方が木の最大の深さよりも小さいときに再現可能
    for left_len in 1..d {
        let right_len = d - left_len;
        let max_depth = left_len.max(right_len);
        if n > max_depth {
            let top = pows[n - max_depth] - 1;
            let left = pows[left_len - 1];
            let right = pows[right_len - 1];
            ans += top * left * right;
            ans %= MOD;
        }
    }

    // NOTE 左右反転を考慮
    ans *= 2;
    ans %= MOD;

    println!("{}", ans);
}
