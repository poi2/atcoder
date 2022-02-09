#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        x: String,
    }
    let digits = x
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect_vec();

    let len = digits.len();
    let mut bits = vec![0; len + 1];
    let mut tmp = 0;

    // NOTE 各桁の和を桁あふれありで計算する
    for i in 0..len {
        tmp += digits[i];
        bits[len - i - 1] = tmp;
    }

    // NOTE 各桁の桁あふれを次の桁に伝搬させる
    for i in 0..len {
        bits[i + 1] += bits[i] / 10;
        bits[i] = bits[i] % 10;
    }

    bits.reverse();

    let ans = bits
        .iter()
        .skip_while(|&&i| i == 0)
        .map(|i| i.to_string())
        .join("");
    println!("{}", ans);
}
