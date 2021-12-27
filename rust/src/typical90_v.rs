// https://atcoder.jp/contests/typical90/tasks/typical90_v
// num-integer crate を使う場合 num_integer のようにアンダースコアになる？
// https://docs.rs/num-integer/0.1.44/num_integer/fn.gcd.html
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn mygcd(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        let z = x % y;
        x = y;
        y = z;
    }
    x
}

fn main() {
    input! {
        mut args: [u64; 3],
    }
    args.sort();
    let ab = mygcd(args[0], args[1]);
    let abc = mygcd(ab, args[2]);

    let ans = args.iter().fold(0, |acc, &arg| acc + arg / abc) - 3;
    println!("{}", ans);
}
