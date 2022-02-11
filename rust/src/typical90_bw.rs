#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
    }
    let facts = factorization(n);
    let prime_count = facts.iter()
        .map(|&(_prime, c)| c )
        .sum::<usize>();

    let mut ans = 0;
    // NOTE 毎回 bit 演算しても処理時間に大差なかった
    // n は 10**12 が最大値なので、prime_count の最大値は 64 以下であることは自明
    // (1..64)iter().sum() 回の計算が追加されたところで処理時間に影響はない
    while prime_count > (1 << ans) {
        ans += 1;
    }
    println!("{}", ans);
}

// NOTE 素因数分解
fn factorization(n: usize) -> Vec<(usize, usize)> {
    fn factor_sub(n: usize, m: usize) -> (usize, usize) {
        let mut c = 0;
        let mut x = n;
        while x % m == 0 {
            c += 1;
            x /= m;
        }
        return (c, x);
    }

    let mut ans = vec![];
    let (c, n) = factor_sub(n, 2);
    if c > 0 { ans.push((2, c)); }

    let mut x = 3;
    let mut m = n;
    while x * x <= m {
        let (c, n) = factor_sub(m, x);
        if c > 0 { ans.push((x, c)); }
        m = n;
        x += 2;
    }
    if m > 1 { ans.push((m, 1)); }

    return ans;
}
