#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::HashMap;

pub struct FenwickTree<T> {
    n: usize,
    ary: Vec<T>,
    e: T,
    mod_num: T,
}

impl<T: Clone + std::ops::AddAssign<T> + std::ops::RemAssign<T>> FenwickTree<T> {
    pub fn new(n: usize, e: T, mod_num: T) -> Self {
        FenwickTree {
            n,
            ary: vec![e.clone(); n],
            e,
            mod_num,
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = self.e.clone();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            // オーバーフローが発生するため、MOD の余りを計算させる
            sum %= self.mod_num.clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    // pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        idx += 1;
        while idx <= n {
            self.ary[idx - 1] += val.clone();
            idx += idx & idx.wrapping_neg();
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum(&self, l: usize, r: usize) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(r) - self.accum(l)
    }
}

fn mod_pow(base: usize, times: usize, mod_num: usize) -> usize {
    if times == 0 { return 1usize; }
    if times == 1 { return base; }
    let temp = mod_pow(base, times/2, mod_num);
    temp * temp % mod_num * mod_pow(base, times%2, mod_num) % mod_num
}

// Ar...Al を固定で考える。
// Al より小さい Ar を見つけだし、その区間の間の数を部分列に追加してもしなくても題意を満たす。
// Ar < Al (r < l) となるとき、間の要素数は l - r - 1 で、部分列は 2 ** (l - r - 1) 個存在する。
// 2 ** (l - r - 1) == 2 ** l * (1/2) ** (r + 1)
// Al 0..n で変化させ、Al 以下となる Ar の r の集合に対して SUM(2 ** l * (1/2) ** (r + 1)) を求める。
// SUM(2 ** l * (1/2) ** (r + 1))
// == 2 ** l * SUM((1/2) ** (r + 1))
// == front * back

// ただこれだと back が float になって正確に計算することができないため逆元を使う。
// しかし、逆元を使った管理が理解できなかった……。
// 逆元の理解が深まったら再度チャレンジする。
// https://atcoder.jp/contests/abc221/editorial/2718

fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }
    let mod_num: usize = 998244353;

    // 1 <= Ai <= 10**9 と大きいため、次元圧縮を行って計算量を減らす
    let mut sorted = an.clone();
    sorted.sort();
    sorted.dedup();
    let mut map = HashMap::new();
    for i in 0..sorted.len() {
        map.insert(sorted[i], i);
    }

    // フェンウィック木を使って区間和を高速に計算する
    let mut fenwick_tree = FenwickTree::new(sorted.len(), 0_usize, mod_num);
    let mut ans = 0;
    let mut current = 1;
    let inverse = mod_pow(2, mod_num - 2, mod_num);
    for i in 0..n {
        let rank = map[&an[i]];
        if i > 0 {
            ans += mod_pow(current, mod_num - 2, mod_num) * fenwick_tree.sum(0, rank + 1);
            ans %= mod_num;
        }

        current *= inverse;
        current %= mod_num;

        fenwick_tree.add(rank, current);
    }
    println!("{}", ans);
}
