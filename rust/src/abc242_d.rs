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
