#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;

fn main() {
    input! {
        n: usize, s: String,
    }
    const MOD: usize = 998244353;
    let max = 1024;
    let list: Vec<usize> = s.chars().map(|c| (c as u8 - 'A' as u8) as usize).collect::<Vec<usize>>();
    let mut dp0 = vec![vec![0; 10]; max];

    for i in 0..n {
        let contest = list[i];
        let mut dp1 = dp0.clone();
        let bit = 1 << contest;
        dp1[bit][contest] += 1;
        dp1[bit][contest] %= MOD;

        for last in 0..10 {
            for used in 0..max {
                // NOTE
                // 1. 未実施のコンテストは実施可能
                // 2. 実施済みであった場合、直前に実施している場合は実施可能
                // 3. どちらでもない場合は実施不可能
                if used & bit > 0 && contest != last { continue }
                let next_used = used | (1 << last);
                dp1[next_used][contest] += dp0[used][last];
                dp1[next_used][contest] %= MOD;
            }
        }
        dp0 = dp1;
    }
    let mut ans = 0;
    for last in 0..10 {
        for used in 0..max {
            ans += dp0[used][last];
            ans %= MOD;
        }
    }
    println!("{}", ans);
}
