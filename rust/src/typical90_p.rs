#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut coints: [usize; 3],
    };
    coints.sort_unstable_by(|a, b| b.cmp(a));

    let a = coints[0];
    let b = coints[1];
    let c = coints[2];

    let mut ans = 10000;

    for i in 0..=n / a {
        let max_j = (n - a * i) / b;
        for j in 0..=max_j {
            let rem = n - (a * i + b * j);
            let k = rem / c;
            if rem % c == 0 {
                ans = ans.min(i + j + k);
            }
        }
    }

    println!("{}", ans);
}
