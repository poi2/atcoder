#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        s: String,
        q: usize,
        tkq: [(u128, u128); q],
    }

    let words = s.chars()
        .map(|c| match c { 'A' => 0, 'B' => 1, _ => 2, } )
        .collect::<Vec<usize>>();
    let base = vec!['A', 'B', 'C'];

    for (t, k) in tkq {
        let (word_idx, increment) = convolution(t, k - 1);
        println!("{}", base[restor_idx(&words, word_idx as usize, increment as usize)]);
    }
}

// return (文字列 S の何番目の文字からの派生か、そこから何文字進んだ文字か)
fn convolution(t: u128, k: u128) -> (u128, u128) {
    if t == 0 { return (k, 0); }
    if k == 0 { return (0, t); }

    let (word_idx, increment) = convolution(t - 1, k / 2);
    return if k % 2 == 0 {
        (word_idx, (increment + 1) % 3)
    } else {
        (word_idx, (increment + 2) % 3)
    };
}

// 最終的に表示される文字列の index を返す
fn restor_idx(words: &Vec<usize>, word_idx: usize, increment: usize) -> usize {
    return (words[word_idx] + increment) % 3;
}

// 検証
// 0 a   b   c
// 1 b c c a a b
// 2 caababbabaca

// 漸化式から dp へ
// if k % 2 == 0; dp[t][k] = (dp[t - 1][k / 2] + 1) % 3;
// if k % 2 != 0; dp[t][k] = (dp[t - 1][k / 2] + 2) % 3;

// 考察
// k /= 2 したあとの数の奇遇によって increment の部分の + 1 or + 2 が変化するので、
// オリジナルの k の bit が立っている数が奇数になる回数（odd）、偶数の回数は even = t - odd となる。
// 最終的な increment は even + odd * 2 になる。

// ki <= min(10**18 or S(ti)) の制約条件から、
// 10 ** 18 == 2 ** x
// <=> 18 + 18 * 2.32 = 59.76 = x
// つまり、t > 60 であれば、S のゼロ文字目の派生で確定する。
// t < 60 であれば、10 ** 18 より S(ti) の方が短いので、S の N 文字目の派生になる可能性がある。

// となると、t -= 1, k /= 2 のどちらが先に 0 に収束するかで派生元の文字列の位置が分かり、
// そこまでの increment の数を数えることで、畳み込みをしなくても計算することができるようになるはず。
