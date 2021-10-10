// https://atcoder.jp/contests/abc219/tasks/abc219_d

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        list: [(usize, usize); n]
    }

    let max = 500;

    let mut dp = vec![vec![vec![max; y + 1]; x + 1]; n + 1];
    dp[0][0][0] = 0;
    for i in 0..n {
        let (a, b) = list[i];
        for j in 0..(x+1) {
            for k in 0..(y+1) {
                let ux = x.min(a + j);
                let uy = y.min(b + k);

                // 購入
                dp[i + 1][ux][uy] = dp[i + 1][ux][uy].min(dp[i][j][k] + 1);
                // 購入しない
                dp[i + 1][j][k] = dp[i + 1][j][k].min(dp[i][j][k]);
            }
        }
    }

    let ans = if dp[n][x][y] == max {
        -1
    } else {
        dp[n][x][y]
    };
    println!("{}", ans);
}