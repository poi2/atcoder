#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use itertools::Itertools;
use num_bigint::BigUint;

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

// 多倍長演算を試したが TLE になった
// x は 10**500,000 なので、多倍長の四則演算を 500,000 回行うと 2sec を超えてしまうようだ
fn main2() {
    input! {
        mut s: String,
    }
    let mut x = BigUint::parse_bytes(s.as_bytes(), 10).unwrap();
    let mut ans = BigUint::parse_bytes(b"0", 10).unwrap();
    let zero = BigUint::parse_bytes(b"0", 10).unwrap();
    let two = BigUint::parse_bytes(b"10", 10).unwrap();

    while x > zero {
        ans += &x;
        x = &x / &two;
    }

    println!("{}", ans.to_str_radix(10));
}