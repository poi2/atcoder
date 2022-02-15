#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use proconio::input;
use num_integer::Roots;

fn main() {
    input! {
        n: usize,
    }

    // y = n / x と第一象限に囲まれる範囲の実数値 (x, y) の交点の数を数えたい。
    // y = x より大きく、y = n / x より小さい交点を one_side で数え、
    // それを２倍して、y = x 上の交点を加算したものが解になる。
    let mut one_side = 0;
    for i in 1..=n.sqrt() {
        one_side += n / i - i;
    }
    println!("{}", one_side * 2 + n.sqrt());
}
