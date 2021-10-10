// https://atcoder.jp/contests/abc200/tasks/abc200_d

use proconio::input;

fn join(a: usize, b: usize) -> usize {
    a * 10 + b
}
fn sep(xy: usize) -> (usize, usize) {
    let x = xy % 10;
    let y = xy / 10;
    ((x + y) % 10, (x * y) % 10)
}

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let limit: usize = 998244353;
    let mut memo: [(usize, usize); 100] = [(0, 0); 100];
    for i in 0..100 {
        memo[i] = sep(i);
    }

    let mut dp: [usize; 10] = [0; 10];
    dp[an[0]] = 1;
    for i in 1..n {
        let mut new_dp: [usize; 10] = [0; 10];
        for x in 0..10 {
            if dp[x] > 0 {
                let key = join(x, an[i]);
                let (s, t) = memo[key];
                new_dp[s] = (new_dp[s] + dp[x]) % limit;
                new_dp[t] = (new_dp[t] + dp[x]) % limit;
            }
        }
        dp = new_dp;
    }
    for i in 0..10 {
        println!("{}", dp[i]);
    }
}
