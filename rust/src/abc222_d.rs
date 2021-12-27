// https://atcoder.jp/contests/abc222/tasks/abc222_d

// use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        an: [usize; n],
        bn: [usize; n],
    }
    let mut dp: [usize; 3001] = [0; 3001];
    dp[0] = 1;
    let div: usize = 998244353;

    for i in 0..n {
        let mut next_dp: [usize; 3001] = [0; 3001];
        let mut count_memo: usize = 0;
        for j in an[i]..bn[i] + 1 {
            let mut count: usize = 0;
            let old = if i == 0 { 0 } else { an[i - 1] };
            if count_memo > 0 {
                count = (count_memo + dp[j]) % div;
            } else {
                for k in old..j + 1 {
                    count = (count + dp[k]) % div;
                }
            }
            count_memo = count;
            next_dp[j] = count;
        }
        dp = next_dp;
    }
    let ans = dp.iter().fold(0, |acc, x| (acc + x) % div);
    println!("{}", ans);
}
